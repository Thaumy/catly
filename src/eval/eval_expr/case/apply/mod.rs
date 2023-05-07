mod primitive_apply;
mod source_lhs_to_closure;
#[cfg(test)]
mod test;

use std::ops::{Deref, Rem};

use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::case::apply::primitive_apply::primitive_apply;
use crate::eval::eval_expr::case::apply::source_lhs_to_closure::source_lhs_to_closure;
use crate::eval::eval_expr::{eval_expr, EvalRet};
use crate::eval::r#macro::false_type;
use crate::eval::r#macro::namely_type;
use crate::eval::r#macro::true_type;
use crate::eval::r#type::eval_err::EvalErr;
use crate::eval::r#type::expr::primitive_op::PrimitiveOp;
use crate::eval::r#type::expr::Expr;
use crate::eval::r#type::r#type::Type;
use crate::infra::either::{Either, EitherAnyExt};
use crate::infra::option::OptionAnyExt;
use crate::infra::r#box::BoxAnyExt;
use crate::infra::result::ResultAnyExt;

pub fn case_apply(
    type_env: &TypeEnv,
    expr_env: Box<ExprEnv>,
    lhs_expr: &Expr,
    rhs_expr: &Expr
) -> EvalRet {
    let lhs_expr = match lhs_expr {
        apply_expr @ Expr::Apply(..) =>
            eval_expr(type_env, expr_env.clone(), apply_expr),
        other => other.clone().ok()
    }?;

    match source_lhs_to_closure(type_env, expr_env.clone(), &lhs_expr)
    {
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
                        rhs_expr.clone(),
                        expr_env.clone()
                    )
                    .boxed(),
                None => output_eval_env.clone()
            };

            eval_expr(type_env, extended_eval_env, &output_expr)
        }
        Either::R((primitive_op, op_eval_env)) => primitive_apply(
            type_env,
            op_eval_env,
            &primitive_op,
            rhs_expr
        )
    }
}
