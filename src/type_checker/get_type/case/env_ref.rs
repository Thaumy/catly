use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::infra::alias::MaybeType;
use crate::infra::quad::Quad;
use crate::type_checker::get_type::r#fn::lift_or_left;
use crate::type_checker::get_type::r#type::GetTypeReturn;
use crate::{has_type, require_constraint, type_miss_match};

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &MaybeType,
    ref_name: &str
) -> GetTypeReturn {
    match expr_env.get_type_with_hint(ref_name, expect_type) {
        // 约束到确切类型, 尝试提升
        Quad::L(t) => match lift_or_left(type_env, &t, expect_type) {
            Some(expect_type) => has_type!(expect_type),
            None => type_miss_match!()
        },
        // 提升并传播约束
        Quad::ML(rc) =>
            match lift_or_left(type_env, &rc.r#type, expect_type) {
                Some(expect_type) =>
                    require_constraint!(expect_type, rc.constraint),
                None => type_miss_match!()
            },
        // 引用源类型信息不足或引用源类型不匹配
        // 引用源类型信息不足, 例如:
        // f -> a -> f a
        // env:
        // def f = _
        // def a = _
        // 无法推导出 a 的类型, 因为 a 的类型是自由的
        mr_r => mr_r
    }
}
