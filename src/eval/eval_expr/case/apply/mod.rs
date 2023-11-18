use std::rc::Rc;

use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::{eval_expr, EvalRet};
use crate::eval::eval_expr::case::apply::r#fn::primitive_apply::primitive_apply;
use crate::eval::eval_expr::case::apply::r#fn::source_lhs_to_closure::source_lhs_expr_to_closure;
use crate::eval::r#type::expr::Expr;
use crate::infra::either::Either;
use crate::infra::option::WrapOption;
use crate::infra::rc::RcAnyExt;

mod r#fn;
#[cfg(test)]
mod test;

pub fn case_apply(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    lhs_expr: &Rc<Expr>,
    rhs_expr: &Rc<Expr>
) -> EvalRet {
    match source_lhs_expr_to_closure(type_env, expr_env, lhs_expr)? {
        Either::L((
            input_name,
            input_type,
            output_expr,
            output_eval_env
        )) => {
            let extended_eval_env = match input_name {
                Some(input_name) => output_eval_env
                    .extend_new(
                        input_name,
                        input_type,
                        rhs_expr.clone().wrap_some(),
                        expr_env.clone().wrap_some()
                    )
                    .rc(),
                None => output_eval_env
            };

            eval_expr(type_env, &extended_eval_env, &output_expr)
        }
        Either::R((ref primitive_op, ref lhs_eval_env)) =>
            primitive_apply(
                type_env,
                lhs_eval_env,
                expr_env,
                primitive_op,
                rhs_expr
            ),
    }
}
