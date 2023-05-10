#[cfg(test)]
mod test;

use std::rc::Rc;

use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::{eval_expr, EvalRet};
use crate::eval::r#type::eval_err::EvalErr;
use crate::infra::result::ResultAnyExt;

pub fn case_env_ref(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    ref_name: &String
) -> EvalRet {
    // TODO:
    // 此处为逐层查找 env_ref
    // 可以设置穿透的访问链, 提高 env_ref 的检索效率
    match expr_env
        .clone()
        .get_src_expr_and_env(ref_name.as_str())
    {
        Some((src_expr, ref src_env)) =>
            eval_expr(type_env, src_env, src_expr),
        None => EvalErr::EnvRefNotFound(format!(
            "EnvRef::{ref_name:?} not found in expr env"
        ))
        .err()
    }
}
