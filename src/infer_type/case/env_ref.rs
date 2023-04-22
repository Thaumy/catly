use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;

// TODO: 外部环境约束同层传播完备性
pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &MaybeType,
    ref_name: &str
) -> InferTypeRet {
    let result = expr_env.infer_type_with_hint(ref_name, expect_type);

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
    // TODO: 在所有类似的地方都应用这种检查
    // TODO: 当提升部分类型时, 此处可以修改 ReqInfo 到 ref_name
    InferTypeRet::from_auto_lift(
        type_env,
        &t,
        expect_type,
        constraint.some()
    )
}
