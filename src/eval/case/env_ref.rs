use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::{eval, EvalRet};

pub fn case_env_ref(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    ref_name: &String
) -> EvalRet {
    let (src_expr, src_env) = expr_env
        .get_expr_and_env(ref_name.as_str())
        .unwrap_or_else(|| {
            panic!("EnvRef::{ref_name:?} not found in expr env")
        });
    eval(type_env, src_env, src_expr)
}
