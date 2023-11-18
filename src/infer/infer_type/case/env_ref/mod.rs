#[cfg(test)]
mod test;

use std::rc::Rc;

use crate::infer::env::ExprEnv;
use crate::infer::env::TypeEnv;
use crate::infer::infer_type::InferTypeRet;
use crate::infra::Triple;
use crate::infra::WrapOption;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::OptType;

pub fn case(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    expect_type: &OptType,
    ref_name: &str
) -> InferTypeRet {
    match expr_env.infer_type_with_hint(
        type_env,
        ref_name,
        expect_type
    )? {
        result @ (Triple::L(_) | Triple::M(_)) => {
            let (typed_src_expr, constraint) =
                result.unwrap_expr_constraint();

            InferTypeRet::from_auto_lift(
                type_env,
                typed_src_expr.unwrap_type_annot(),
                expect_type,
                constraint.wrap_some(),
                |t| Expr::EnvRef(t.wrap_some(), ref_name.to_string())
            )
        }
        // Triple::R
        ref_type => {
            // 引用源类型信息不足
            // 引用源类型信息不足, 例如:
            // f -> a -> f a
            // env:
            // def f = _
            // def a = _
            // 无法推导出 a 的类型, 因为 a 的类型是自由的
            ref_type.into()
        }
    }
}
