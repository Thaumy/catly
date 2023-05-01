use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::r#type::env_ref_src::EnvRefSrc;
use crate::infer::env::r#type::type_constraint::TypeConstraint;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#fn::has_type;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::require_constraint::require_extended_constraint;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::option::OptionAnyExt;
use crate::infra::quad::Quad;
use crate::parser::r#type::r#type::OptType;

impl<'t> ExprEnv<'t> {
    // 由于表达式对环境中的 ref_name 的使用是一种间接使用, 可能存在多处引用对于 ref_name 的类型要求不一致的情况
    // 所以如果 hint 发挥了作用, 那么一定要对 ref_name 产生到 hint 的约束
    // 暂不考虑泛型(类型参数)设计, 类型参数允许多个不同的约束作用于同一表达式, 这与现有的约束设计冲突
    pub fn infer_type_with_hint<'s>(
        &self,
        type_env: &TypeEnv,
        ref_name: impl Into<&'s str> + Clone,
        hint: &OptType
    ) -> InferTypeRet {
        let ref_type = self.infer_type(type_env, ref_name.clone());

        match ref_type.clone() {
            // HACK:
            // 特例, 分支约束共享会导致向环境中注入无源不完整类型
            // 当 hint 为完整类型时可进行反向提升
            // TODO: 考虑对不完整类型的提升规则, 这些规则有助于进一步明确类型信息
            Quad::L(ref_type) if ref_type.is_partial() =>
                match hint {
                    Some(hint) if !hint.is_partial() =>
                        InferTypeRet::from_auto_lift(
                            type_env,
                            hint,
                            &ref_type.clone().some(),
                            // 需要将不完整类型约束到精确类型
                            EnvRefConstraint::single(ref_name.into(), hint.clone()).some(),
                        ),
                    _ => has_type(ref_type)
                }
            // 缺乏类型信息, 尝试提示
            Quad::MR(ri) if let Some(hint) = hint => {
                let constraint_acc = ri.constraint;
                let tc_and_src = self
                    .find_entry(ref_name.clone().into())
                    .map(|(_, tc, src)| (tc, src));

                match tc_and_src {
                    // 环境中不存在引用名
                    None => require_extended_constraint(
                        hint.clone(),
                        constraint_acc,
                        EnvRefConstraint::single(
                            ref_name.into(),
                            hint.clone(),
                        ),
                    ),
                    // 引用名自由无源
                    Some((
                             TypeConstraint::Free,
                             EnvRefSrc::NoSrc
                         )) => require_extended_constraint(
                        hint.clone(),
                        constraint_acc,
                        EnvRefConstraint::single(
                            ref_name.into(),
                            hint.clone(),
                        ),
                    ),
                    // 引用名自由有源, 且引用源无类型标注
                    // 如果 hint 有效, 应对 ref_name 产生到 hint 的约束
                    Some((
                             TypeConstraint::Free,
                             EnvRefSrc::Src(src_expr)
                         )) if src_expr.is_no_type_annot() =>
                        match src_expr
                            .with_fallback_type(hint)
                            .infer_type(type_env, self)
                        {
                            // 因为此时无类型标注, 所以得到的类型一定是 hint 或更完整的 hint
                            // 将 ref_name 约束到类型结果 t 不会导致约束类型不一致
                            // 因为 t 要么比 hint 更加完整, 要么等于 hint
                            Quad::L(t) =>
                                require_extended_constraint(
                                    t.clone(),
                                    constraint_acc,
                                    EnvRefConstraint::single(
                                        ref_name.into(),
                                        t,
                                    ),
                                ),
                            Quad::ML(rc) => {
                                let ref_name_constraint =
                                    EnvRefConstraint::single(
                                        ref_name.into(),
                                        rc.r#type.clone(),
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
                                            &ref_name_constraint,
                                        )
                                            .into(),
                                }
                            }
                            mr_r => mr_r
                        }
                    _ => ref_type
                }
            }
            _ => ref_type
        }
    }
}
