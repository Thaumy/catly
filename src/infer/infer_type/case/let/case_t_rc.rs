use std::rc::Rc;

use crate::infer::env::ExprEnv;
use crate::infer::env::TypeEnv;
use crate::infer::infer_type::InferTypeRet;
use crate::infra::Triple;
use crate::infra::WrapOption;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::OptType;
use crate::parser::r#type::Type;

pub fn case_t_rc<F>(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    expect_type: &OptType,
    scope_expr: &Expr,
    typed_expr_cons: F
) -> InferTypeRet
where
    F: Fn(Type, Expr) -> Expr
{
    // Hint scope_expr with expect_type and get scope_expr_type
    match scope_expr
        .with_opt_fallback_type(expect_type)
        .infer_type(type_env, expr_env)?
    {
        result @ (Triple::L(_) | Triple::M(_)) => {
            let (typed_scope_expr, constraint) =
                result.unwrap_expr_constraint();
            InferTypeRet::from_auto_lift(
                type_env,
                typed_scope_expr.unwrap_type_annot(),
                expect_type,
                constraint.wrap_some(),
                |t| typed_expr_cons(t, typed_scope_expr.clone())
            )
        }

        Triple::R(ri) => ri.into()
    }
}
