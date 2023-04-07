use crate::infra::alias::MaybeType;
use crate::type_checker::get_type::r#fn::{
    find_ref_type,
    lift_or_left
};
use crate::type_checker::get_type::r#type::{
    ExprEnv,
    GetTypeReturn,
    TypeConstraint,
    TypeEnv
};
use crate::{
    has_type,
    require_constraint,
    require_info,
    type_miss_match
};

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    t: &MaybeType,
    ref_name: &str
) -> GetTypeReturn {
    // 直接获取环境类型, 不再进行推导
    match find_ref_type(expr_env, ref_name) {
        None => match t {
            // 环境类型缺失, 但可以通过建立约束修复
            Some(t) =>
                return require_constraint!(t.clone(), vec![(
                    ref_name.to_string(),
                    t.clone()
                )]),
            // 缺乏推导信息
            None => return require_info!(ref_name.to_string())
        },
        // 成功获取到环境类型
        Some(ref_type) => match ref_type {
            TypeConstraint::Free => match t {
                Some(t) => require_constraint!(t.clone(), vec![(
                    ref_name.to_string(),
                    t.clone()
                )]),
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
                match lift_or_left(type_env, ct, t) {
                    Some(t) => has_type!(t),
                    None => type_miss_match!()
                },
        }
    }
}
