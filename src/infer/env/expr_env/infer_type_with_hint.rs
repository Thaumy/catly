use std::rc::Rc;

use crate::infer::env::EnvRefSrc;
use crate::infer::env::ExprEnv;
use crate::infer::env::TypeConstraint;
use crate::infer::env::TypeEnv;
use crate::infer::infer_type::require_extended_constraint;
use crate::infer::infer_type::EnvRefConstraint;
use crate::infer::infer_type::InferTypeRet;
use crate::infer::infer_type::TypeMissMatch;
use crate::infra::Triple;
use crate::infra::WrapOption;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::OptType;

impl ExprEnv {
    // 由于表达式对环境中的 ref_name 的使用是一种间接使用, 可能存在多处引用对于 ref_name 的类型要求不一致的情况
    // 所以如果 hint 发挥了作用, 那么一定要对 ref_name 产生到 hint 的约束
    // 暂不考虑泛型(类型参数)设计, 类型参数允许多个不同的约束作用于同一表达式, 这与现有的约束设计冲突
    pub fn infer_type_with_hint<'s>(
        self: &Rc<Self>,
        type_env: &TypeEnv,
        ref_name: impl Into<&'s str> + Clone,
        hint: &OptType
    ) -> InferTypeRet {
        let ref_name = ref_name.into().to_string();
        let ref_type =
            self.infer_type(type_env, ref_name.as_str())?;

        match ref_type.clone() {
            // HACK:
            // 特例, 分支约束共享会导致向环境中注入无源不完整类型
            // 当 hint 为完整类型时可进行反向提升
            // TODO: 考虑对不完整类型的提升规则, 这些规则有助于进一步明确类型信息
            Triple::L(typed_expr)
                if typed_expr
                    .clone()
                    .unwrap_type_annot()
                    .is_partial() =>
            {
                let ref_type = typed_expr.unwrap_type_annot();
                match hint {
                    Some(hint) if !hint.is_partial() =>
                        InferTypeRet::from_auto_lift(
                            type_env,
                            hint,
                            &ref_type.clone().wrap_some(),
                            // 需要将不完整类型约束到精确类型
                            EnvRefConstraint::single(
                                ref_name.clone(),
                                hint.clone()
                            )
                            .wrap_some(),
                            // 与 infer_type 同理
                            |t| {
                                Expr::EnvRef(
                                    t.wrap_some(),
                                    ref_name.clone()
                                )
                            }
                        ),
                    _ => InferTypeRet::has_type(Expr::EnvRef(
                        ref_type.clone().wrap_some(),
                        ref_name
                    ))
                }
            }
            // 缺乏类型信息, 尝试提示
            Triple::R(ri) if let Some(hint) = hint => {
                let constraint_acc = ri.constraint;
                let tc_and_src = self
                    .find_entry(ref_name.as_str())
                    .map(|(_, tc, src)| (tc, src));

                match tc_and_src {
                    // 环境中不存在引用名
                    None => require_extended_constraint(
                        Expr::EnvRef(hint.clone().wrap_some(), ref_name.clone()),
                        constraint_acc,
                        EnvRefConstraint::single(
                            ref_name,
                            hint.clone()
                        )
                    ),
                    // 引用名自由无源
                    Some((
                        TypeConstraint::Free,
                        EnvRefSrc::NoSrc
                    )) => require_extended_constraint(
                        Expr::EnvRef(hint.clone().wrap_some(), ref_name.clone()),
                        constraint_acc,
                        EnvRefConstraint::single(
                            ref_name,
                            hint.clone()
                        )
                    ),
                    // 引用名自由有源, 且引用源无类型标注
                    // 如果 hint 有效, 应对 ref_name 产生到 hint 的约束
                    Some((
                        TypeConstraint::Free,
                        EnvRefSrc::Src(src_expr)
                    )) if src_expr.is_no_type_annot() =>
                        match src_expr
                            .with_fallback_type(hint)
                            .infer_type(type_env, self)?
                        {
                            // 因为此时无类型标注, 所以得到的类型一定是 hint 或更完整的 hint
                            // 将 ref_name 约束到类型结果 t 不会导致约束类型不一致
                            // 因为 t 要么比 hint 更加完整, 要么等于 hint
                            Triple::L(typed_expr) => {
                                let t =
                                    typed_expr.unwrap_type_annot();
                                require_extended_constraint(
                                    Expr::EnvRef(
                                        t.clone().wrap_some(),
                                        ref_name.clone()
                                    ),
                                    constraint_acc,
                                    EnvRefConstraint::single(
                                        ref_name,
                                        t.clone()
                                    )
                                )
                            }
                            Triple::M(rc) => {
                                let ref_name_constraint =
                                    EnvRefConstraint::single(
                                        ref_name,
                                        rc.typed_expr.unwrap_type_annot().clone()
                                    );

                                match constraint_acc.extend_new(
                                    ref_name_constraint.clone()
                                ) {
                                    Some(constraint) => rc
                                        .with_constraint_acc(
                                            constraint
                                        ),
                                    None =>
                                        TypeMissMatch::of_constraint(
                                            &constraint_acc,
                                            &ref_name_constraint
                                        )
                                        .into(),
                                }
                            }
                            ri => ri.into()
                        },
                    _ => ref_type.into()
                }
            }
            _ => ref_type.into()
        }
    }
}
