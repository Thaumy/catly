use crate::eval::env::expr_env::ExprEnv;
use crate::eval::r#type::eval_err::EvalErr;
use crate::eval::r#type::r#type::Type;
use crate::eval::{eval, EvalRet};
use crate::infra::result::AnyExt;

pub fn case_env_ref(
    expr_env: &ExprEnv,
    ref_name: &String
) -> EvalRet {
    let src_expr = expr_env.get_expr(ref_name.as_str());
    match src_expr {
        Some(src_expr) => eval(expr_env, src_expr),
        None =>
            return EvalErr::of(format!(
                "EnvRef {ref_name:?} not found in scope"
            ))
            .err(),
    }
}
