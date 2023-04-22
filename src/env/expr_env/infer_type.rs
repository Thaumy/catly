use crate::env::expr_env::ExprEnv;
use crate::env::r#type::env_ref_src::EnvRefSrc;
use crate::env::r#type::type_constraint::TypeConstraint;
use crate::infer_type::r#fn::{
    has_type,
    lift_or_miss_match,
    with_constraint_lift_or_miss_match
};
use crate::infer_type::r#type::require_constraint::require_constraint;
use crate::infer_type::r#type::require_info::RequireInfo;
use crate::infer_type::r#type::GetTypeReturn;
use crate::infra::quad::Quad;

impl<'t> ExprEnv<'t> {
    pub fn infer_type(&self, ref_name: &str) -> GetTypeReturn {
        let tc_and_src = self
            .find_entry(ref_name)
            .map(|(_, tc, src)| (tc, src));

        match tc_and_src {
            // 当前环境查找到引用名, 但不存在引用源
            Some((tc, EnvRefSrc::NoSrc)) => match tc {
                // 引用名所对应的类型是类型约束的直接类型
                TypeConstraint::Constraint(t) => has_type(t.clone()),
                // 不存在类型约束
                TypeConstraint::Free =>
                    RequireInfo::of(ref_name).into(),
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
                    TypeConstraint::Constraint(t) => match src_expr
                        .with_fallback_type(t)
                        .infer_type(&self.type_env, &new_expr_env)
                    {
                        Quad::L(src_expr_type) => lift_or_miss_match(
                            &self.type_env,
                            &src_expr_type,
                            &t
                        ),
                        Quad::ML(rc) =>
                            with_constraint_lift_or_miss_match(
                                rc.constraint,
                                &self.type_env,
                                &rc.r#type,
                                &t
                            ),
                        // 如果引用源是无类型弃元
                        Quad::MR(ri) if ri.ref_name == "_" =>
                        // 为了防止无类型弃元信息被捕获, 改写错误信息
                            RequireInfo::of(ref_name).into(),
                        // 无法处理其他情况
                        mr_r => mr_r
                    },

                    // 不具备约束
                    TypeConstraint::Free => match src_expr
                        .infer_type(&self.type_env, &new_expr_env)
                    {
                        Quad::L(src_expr_type) =>
                            has_type(src_expr_type),
                        // 由于 ref_name 是 Free 的, 所以此时约束可能作用于 ref_name 本身
                        // 此时作用于 ref_name 的约束相当于 ref_name 的固有类型, 只需将其他约束按需传播
                        // 如果引用源是无类型弃元
                        Quad::ML(rc) => require_constraint(
                            rc.r#type,
                            rc.constraint
                                .exclude_new(ref_name)
                        ),
                        Quad::MR(ri) if ri.ref_name == "_" =>
                        // 为了防止无类型弃元信息被捕获, 改写错误信息
                            RequireInfo::of(ref_name).into(),
                        // 类型不相容
                        mr_r => mr_r
                    }
                }
            }

            // 缺乏推导信息
            None => RequireInfo::of(ref_name).into()
        }
    }
}
