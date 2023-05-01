#[cfg(test)]
mod test;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infra::option::OptionAnyExt;
use crate::infra::quad::Quad;
use crate::parser::r#type::r#type::OptType;

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &OptType,
    ref_name: &str
) -> InferTypeRet {
    let result = expr_env.infer_type_with_hint(
        type_env,
        ref_name,
        expect_type
    );

    if let Quad::R(_) | Quad::MR(_) = result {
        // 引用源类型信息不足或引用源类型不匹配
        // 引用源类型信息不足, 例如:
        // f -> a -> f a
        // env:
        // def f = _
        // def a = _
        // 无法推导出 a 的类型, 因为 a 的类型是自由的
        return result;
    };

    let (t, constraint) = result.unwrap_type_and_constraint();

    InferTypeRet::from_auto_lift(
        type_env,
        &t,
        expect_type,
        constraint.some()
    )
}
