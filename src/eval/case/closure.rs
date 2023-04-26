use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::r#type::expr::Expr;
use crate::eval::r#type::r#type::Type;
use crate::eval::{eval, EvalRet};

pub fn case_closure(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    output_expr: &Expr
) -> EvalRet {
    eval(type_env, expr_env, output_expr)
}
