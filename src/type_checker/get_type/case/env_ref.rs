use crate::infra::alias::MaybeType;
use crate::type_checker::env::expr_env::ExprEnv;
use crate::type_checker::env::type_env::TypeEnv;
use crate::type_checker::get_type::r#fn::lift_or_left;
use crate::type_checker::get_type::r#type::GetTypeReturn;
use crate::type_checker::r#type::TypeConstraint;
use crate::{
    has_type,
    require_constraint,
    require_info,
    type_miss_match
};

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &MaybeType,
    ref_name: &str
) -> GetTypeReturn {
    match expr_env.find_type(ref_name) {
        None => match expect_type {
            // 环境类型缺失, 但可以通过建立约束修复
            Some(expect_type) =>
                return require_constraint!(
                    expect_type.clone(),
                    vec![(ref_name.to_string(), expect_type.clone())]
                ),
            // 缺乏推导信息
            None => return require_info!(ref_name.to_string())
        },
        // 成功获取到环境类型
        Some(ref_type) => match ref_type {
            TypeConstraint::Free => match expect_type {
                Some(expect_type) => require_constraint!(
                    expect_type.clone(),
                    vec![(ref_name.to_string(), expect_type.clone())]
                ),
                // 信息不足以进行类型推导
                // 例如:
                // f -> a -> f a
                // env:
                // def f = _
                // def a = _
                // 无法推导出 a 的类型, 因为 a 的类型是自由的
                None => require_info!(ref_name.to_string())
            },
            TypeConstraint::Constraint(ct) =>
                match lift_or_left(type_env, ct, expect_type) {
                    Some(expect_type) => has_type!(expect_type),
                    None => type_miss_match!()
                },
        }
    }
}
