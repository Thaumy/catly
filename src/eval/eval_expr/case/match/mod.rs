mod r#fn;

use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::case::r#match::r#fn::is_expr_match_pattern_then_env;
use crate::eval::eval_expr::{eval_expr, EvalRet};
use crate::eval::r#type::eval_err::EvalErr;
use crate::eval::r#type::expr::Expr;
use crate::infra::result::AnyExt as ResAnyExt;

pub fn case_match(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    target_expr: &Expr,
    case_vec: &Vec<(Expr, Expr)>
) -> EvalRet {
    let target_value = eval_expr(type_env, expr_env, target_expr)?;

    case_vec
        .iter()
        .map(|(case_expr, then_expr)| {
            (
                is_expr_match_pattern_then_env(
                    expr_env,
                    &target_value,
                    case_expr
                ),
                then_expr
            )
        })
        .find(|x| matches!(x, (Some(_), _)))
        .map(|(env, then_expr)| {
            eval_expr(type_env, &env.unwrap(), then_expr)
        })
        .unwrap_or_else(|| {
            EvalErr::of(format!(
                "Non-exhaustive match expr cases: {case_vec:?}"
            ))
            .err()
        })
}
