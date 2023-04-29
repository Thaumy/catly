#[cfg(test)]
mod test;

use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::{eval_expr, EvalRet};
use crate::eval::r#type::expr::Expr;
use crate::eval::r#type::r#type::Type;

pub fn case_cond(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    bool_expr: &Expr,
    then_expr: &Expr,
    else_expr: &Expr
) -> EvalRet {
    match eval_expr(type_env, expr_env, bool_expr) {
        Ok(value) => match value {
            Expr::Int(Type::NamelyType(n), 1) if n == "True" =>
                eval_expr(type_env, expr_env, then_expr),
            Expr::Int(Type::NamelyType(n), 0) if n == "False" =>
                eval_expr(type_env, expr_env, else_expr),
            _ => panic!("Impossible bool value: {value:?}")
        },
        e => e
    }
}
