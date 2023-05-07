#[cfg(test)]
mod test;

use std::rc::Rc;

use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::{eval_expr, EvalRet};

pub fn case_env_ref(
    type_env: &TypeEnv,
    expr_env: Rc<ExprEnv>,
    ref_name: &String
) -> EvalRet {
    // TODO:
    // 此处为逐层查找 env_ref
    // 可以设置穿透的访问链, 提高 env_ref 的检索效率
    let (src_expr, src_env) = expr_env
        .get_src_expr_and_env(ref_name.as_str())
        .unwrap_or_else(|| {
            panic!("EnvRef::{ref_name:?} not found in expr env")
        });

    eval_expr(type_env, src_env, src_expr)
}
