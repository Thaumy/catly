#[cfg(test)]
mod test;

use std::rc::Rc;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infra::option::OptionAnyExt;
use crate::infra::triple::Triple;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::OptType;

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
                constraint.some(),
                |t| Expr::EnvRef(t.some(), ref_name.to_string())
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
            return ref_type.into();
        }
    }
}
