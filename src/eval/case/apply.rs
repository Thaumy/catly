use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::r#type::expr::Expr;
use crate::eval::r#type::r#type::Type;
use crate::eval::{eval, EvalRet};

pub fn destruct_env_ref_to_closure<'t>(
    expr_env: &'t ExprEnv,
    expr: &Expr
) -> (Option<String>, Type, Expr, ExprEnv<'t>) {
    match expr {
        Expr::EnvRef(_, ref_name) => {
            let (src_expr, src_env) = expr_env
                .get_expr_and_env(ref_name.as_str())
                .unwrap_or_else(|| {
                    panic!(
                        "EnvRef {ref_name:?} not found in expr env"
                    )
                });

            destruct_env_ref_to_closure(src_env, src_expr)
        }
        Expr::Closure(_, input_name, input_type, output_expr) => (
            input_name.clone(),
            input_type.clone(),
            *output_expr.clone(),
            expr_env.clone()
        ),
        _ => panic!("Impossible expr: {expr:?}")
    }
}

pub fn case_apply(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    lhs_expr: &Expr,
    rhs_expr: &Expr
) -> EvalRet {
    let (input_name, input_type, output_expr, output_eval_env) =
        destruct_env_ref_to_closure(expr_env, lhs_expr);

    let extended_eval_env = match input_name {
        Some(input_name) => output_eval_env.extend_new(
            input_name,
            input_type,
            rhs_expr.clone(),
            output_eval_env.clone()
        ),
        None => output_eval_env
    };

    eval(type_env, &extended_eval_env, &output_expr)
}
