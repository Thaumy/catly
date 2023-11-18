#[cfg(test)]
mod test;

use std::rc::Rc;

use crate::eval::env::ExprEnv;
use crate::eval::env::TypeEnv;
use crate::eval::eval_expr::{eval_expr, EvalRet};
use crate::eval::Expr;
use crate::eval::Type;
use crate::infra::RcAnyExt;
use crate::infra::WrapOption;

pub fn case_let(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    rec_assign: &bool,
    assign_name: &String,
    assign_type: &Type,
    assign_expr: &Rc<Expr>,
    scope_expr: &Rc<Expr>
) -> EvalRet {
    let new_expr_env = if *rec_assign {
        expr_env.extend_new(
            assign_name,
            assign_type.clone(),
            assign_expr
                .clone()
                .wrap_some(),
            None
        )
    } else {
        expr_env.clone().extend_new(
            assign_name,
            assign_type.clone(),
            assign_expr
                .clone()
                .wrap_some(),
            expr_env.clone().wrap_some()
        )
    }
    .rc();

    eval_expr(type_env, &new_expr_env, scope_expr)
}
