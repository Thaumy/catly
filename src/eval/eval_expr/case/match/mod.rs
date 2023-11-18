mod r#fn;
pub use r#fn::*;

#[cfg(test)]
mod test;

use std::rc::Rc;

use crate::eval::env::ExprEnv;
use crate::eval::env::TypeEnv;
use crate::eval::eval_expr::case::r#match::is_expr_match_pattern_then_env;
use crate::eval::eval_expr::{eval_expr, EvalRet};
use crate::eval::EvalErr;
use crate::eval::Expr;
use crate::infra::option::WrapOption;
use crate::infra::rc::RcAnyExt;
use crate::infra::result::WrapResult;

pub fn case_match(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    target_expr: &Rc<Expr>,
    case_vec: &Vec<(Rc<Expr>, Rc<Expr>)>
) -> EvalRet {
    let evaluated_target_expr =
        eval_expr(type_env, expr_env, target_expr)?.rc();

    case_vec
        .iter()
        .map(|(case_expr, then_expr)| {
            (
                is_expr_match_pattern_then_env(
                    type_env,
                    expr_env,
                    &evaluated_target_expr,
                    case_expr
                ),
                then_expr
            )
        })
        .find(|x| matches!(x, (Some(_), _)))
        .and_then(|(env, then_expr)| {
            eval_expr(type_env, &env?, then_expr).wrap_some()
        })
        .unwrap_or_else(|| {
            EvalErr::NonExhaustiveMatch(format!(
                "Non-exhaustive match expr cases: {case_vec:?}"
            ))
            .wrap_err()
        })
}
