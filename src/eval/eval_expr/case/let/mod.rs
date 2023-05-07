#[cfg(test)]
mod test;

use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::{eval_expr, EvalRet};
use crate::eval::r#type::expr::Expr;
use crate::eval::r#type::r#type::Type;
use crate::infra::r#box::BoxAnyExt;

pub fn case_let(
    type_env: &TypeEnv,
    expr_env: Box<ExprEnv>,
    assign_name: &String,
    assign_type: &Type,
    assign_expr: &Expr,
    scope_expr: &Expr
) -> EvalRet {
    let new_expr_env = expr_env
        .extend_new(
            assign_name,
            assign_type.clone(),
            assign_expr.clone(),
            expr_env.clone()
        )
        .boxed();

    eval_expr(type_env, new_expr_env, scope_expr)
}
