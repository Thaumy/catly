use crate::env::expr_env::ExprEnv;
use crate::env::r#type::env_ref_src::EnvRefSrc;
use crate::env::r#type::type_constraint::TypeConstraint;
use crate::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer_type::r#type::require_constraint::require_constraint;
use crate::infra::alias::MaybeType;
use crate::infra::quad::Quad;

impl<'t> ExprEnv<'t> {
    // 由于表达式对环境中的 ref_name 的使用是一种间接使用, 可能存在多处引用对于 ref_name 的类型要求不一致的情况
    // 所以如果 hint 发挥了作用, 那么一定要对 ref_name 产生到 hint 的约束
    // 暂不考虑泛型(类型参数)设计, 类型参数允许多个不同的约束作用于同一表达式, 这与现有的约束设计冲突
    pub fn infer_type_with_hint(
        &self,
        ref_name: &str,
        hint: &MaybeType
    ) -> InferTypeRet {
        let ref_type = self.infer_type(ref_name);
        match ref_type {
            // 缺乏类型信息, 尝试提示
            Quad::MR(_) if let Some(hint) = hint => {
                let tc_and_src = self
                    .find_entry(ref_name)
                    .map(|(_, tc, src)| (tc, src));

                match tc_and_src {
                    // 环境中不存在引用名
                    None =>
                        require_constraint(
                            hint.clone(),
                            EnvRefConstraint::single(
                                ref_name.to_string(),
                                hint.clone(),
                            ),
                        ),
                    // 引用名自由无源
                    Some((TypeConstraint::Free, EnvRefSrc::NoSrc)) =>
                        require_constraint(
                            hint.clone(),
                            EnvRefConstraint::single(
                                ref_name.to_string(),
                                hint.clone(),
                            ),
                        ),
                    // 引用名自由有源, 且引用源无类型标注
                    // 如果 hint 有效, 应对 ref_name 产生到 hint 的约束
                    Some((TypeConstraint::Free, EnvRefSrc::Src(src_expr))) if src_expr.is_no_type_annot() =>
                        match src_expr
                            .with_fallback_type(hint)
                            .infer_type(
                                &self.type_env,
                                self,
                            )
                        {
                            // 因为此时无类型标注, 所以得到的类型一定是 hint 或更完整的 hint
                            // 将 ref_name 约束到类型结果 t 不会导致约束类型不一致
                            // 因为 t 要么比 hint 更加完整, 要么等于 hint
                            Quad::L(t) => require_constraint(
                                t.clone(),
                                EnvRefConstraint::single(
                                    ref_name.to_string(),
                                    t,
                                ),
                            ),
                            Quad::ML(rc) =>
                                rc.with_constraint_acc(
                                    EnvRefConstraint::single(
                                        ref_name.to_string(),
                                        rc.r#type.clone(),
                                    ),
                                ),
                            mr_r => mr_r
                        }
                    _ => ref_type
                }
            }
            _ => ref_type
        }
    }
}
