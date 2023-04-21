use crate::env::expr_env::ExprEnv;
use crate::env::r#type::env_ref_src::EnvRefSrc;
use crate::env::r#type::type_constraint::TypeConstraint;
use crate::get_type::r#fn::{
    has_type,
    lift_or_miss_match,
    require_constraint_or_type,
    with_constraint_lift_or_miss_match
};
use crate::get_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::get_type::r#type::require_constraint::{
    require_constraint,
    require_extended_constraint
};
use crate::get_type::r#type::require_info::RequireInfo;
use crate::get_type::r#type::GetTypeReturn;
use crate::get_type::{get_type, get_type_with_hint};
use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;

impl<'t> ExprEnv<'t> {
    // 由于表达式对环境中的 ref_name 的使用是一种间接使用, 可能存在多处引用对于 ref_name 的类型要求不一致的情况
    // 所以如果 hint 发挥了作用, 那么一定要对 ref_name 产生到 hint 的约束
    // 暂不考虑泛型(类型参数)设计, 类型参数允许多个不同的约束作用于同一表达式, 这与现有的约束设计冲突
    pub fn get_type_with_hint(
        &self,
        ref_name: &str,
        hint: &MaybeType
    ) -> GetTypeReturn {
        match self
            .find_entry(ref_name)
            .map(|(_, tc, src)| (tc, src))
        {
            // 当前环境查找到引用名, 但不存在引用源
            Some((tc, EnvRefSrc::NoSrc)) => match tc {
                // 引用名所对应的类型是类型约束的直接类型
                TypeConstraint::Constraint(t) => has_type(t.clone()),
                // 不存在类型约束
                TypeConstraint::Free => match hint {
                    // 如果有 hint, 则将 ref_name 约束到 hint
                    Some(hint) => require_constraint(
                        hint.clone(),
                        EnvRefConstraint::single(
                            ref_name.to_string(),
                            hint.clone()
                        )
                    ),
                    None => match hint {
                        // 环境约束缺失, 但可以通过建立约束修复
                        Some(hint) => require_constraint(
                            hint.clone(),
                            EnvRefConstraint::single(
                                ref_name.to_string(),
                                hint.clone()
                            )
                        ),
                        // 缺乏推导信息
                        None =>
                            RequireInfo::of(ref_name).into():
                                GetTypeReturn,
                    }
                }
            },

            // 当前环境查找到引用名, 且存在引用源
            Some((tc, EnvRefSrc::Src(src_expr))) => {
                // 为防止递归调用导致类型检查不能终止, 需要将引用名去源后注入环境
                let new_expr_env = self.extend_new(
                    ref_name.to_string(),
                    tc.clone().into(),
                    None
                );

                match tc {
                    // 具备约束
                    TypeConstraint::Constraint(t) => match get_type_with_hint(
                        &self.type_env,
                        &new_expr_env,
                        src_expr,
                        &t.clone().some(),
                    ) {
                        Quad::L(src_expr_type) => lift_or_miss_match(
                            &self.type_env,
                            &src_expr_type,
                            &t,
                        ),
                        Quad::ML(rc) => with_constraint_lift_or_miss_match(
                            rc.constraint,
                            &self.type_env,
                            &rc.r#type,
                            &t,
                        ),
                        // 如果引用源是无类型弃元
                        Quad::MR(ri) if ri.ref_name == "_" => match hint {
                            // 具备 hint, 可以将引用名约束到 hint, 传播该约束
                            Some(hint) => require_constraint(
                                    hint.clone(),
                                    EnvRefConstraint::single(
                                        ref_name.to_string(),
                                        hint.clone()
                                    )
                                ),
                            // 不具备 hint, 为了防止无类型弃元信息被捕获, 改写错误信息
                            None => RequireInfo::of(ref_name).into()
                        },
                        // 无法处理其他情况
                        mr_r => mr_r
                    }

                    // 不具备约束
                    TypeConstraint::Free => match get_type(&self.type_env, &new_expr_env, src_expr) {
                        Quad::L(src_expr_type) => has_type(src_expr_type),
                        // 由于 ref_name 是 Free 的, 所以此时约束可能作用于 ref_name 本身
                        // 此时作用于 ref_name 的约束相当于 ref_name 的固有类型, 只需将其他约束按需传播
                        // 如果引用源是无类型弃元
                        Quad::ML(rc) => require_constraint_or_type(
                            rc.constraint.filter_new(|(n, _)| n != ref_name),
                            rc.r#type,
                        ),
                        Quad::MR(ri) if ri.ref_name == "_" => match hint {
                            // 具备 hint, 可以将引用名约束到 hint, 传播该约束
                            Some(hint) => require_constraint(
                                    hint.clone(),
                                    EnvRefConstraint::single(
                                        ref_name.to_string(),
                                        hint.clone()
                                    )
                                ),
                            // 不具备 hint, 为了防止无类型弃元信息被捕获, 改写错误信息
                            None =>RequireInfo::of(ref_name).into()
                        },
                        // 缺乏约束信息且引用源无类型标注, 此时应使用 hint, 并对 ref_name 产生到 hint 的约束
                        Quad::MR(_) if let Some(hint) = hint && src_expr.is_no_type_annot()
                        => match get_type_with_hint(
                            &self.type_env,
                            self,
                            src_expr,
                            &hint.clone().some(),
                        ) {
                            // 因为此时无类型标注, 所以得到的类型一定是 hint 或更完整的 hint
                            // 将 ref_name 约束到类型结果 t 不会导致约束类型不一致
                            // 因为 t 要么比 hint 更加完整, 要么等于 hint
                            Quad::L(t) => require_constraint(
                                t.clone(),
                                EnvRefConstraint::single(
                                    ref_name.to_string(),
                                    t
                                )
                            ),
                            // TODO: bad fmt
                            Quad::ML(rc)=>
                                require_extended_constraint(
                                    rc.r#type.clone(),
                                    EnvRefConstraint::single(
                                        ref_name.to_string(),
                                        rc.r#type
                                    ),
                                    rc.constraint
                                ),
                            mr_r => mr_r
                        }
                        // 类型不相容
                        r => r
                    },
                }
            }

            None => match hint {
                // 环境约束缺失, 但可以通过建立约束修复
                Some(hint) => require_constraint(
                    hint.clone(),
                    EnvRefConstraint::single(
                        ref_name.to_string(),
                        hint.clone()
                    )
                ),
                // 缺乏推导信息
                None => RequireInfo::of(ref_name).into()
            }
        }
    }
}
