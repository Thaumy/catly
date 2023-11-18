#[cfg(test)]
mod test;

use std::rc::Rc;

use crate::eval::env::ExprEnv;
use crate::eval::env::TypeEnv;
use crate::eval::eval_expr::{eval_expr, EvalRet};
use crate::eval::EvalErr;
use crate::eval::Type;
use crate::eval::{Expr, StructField};
use crate::infra::VecExt;
use crate::infra::WrapRc;
use crate::infra::WrapResult;

pub fn case_struct(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    type_annot: &Type,
    struct_vec: &[StructField]
) -> EvalRet {
    let struct_vec = struct_vec
        .iter()
        .map(|(sf_n, sf_t, sf_e)| {
            (
                sf_n.clone(),
                sf_t.clone(),
                eval_expr(type_env, expr_env, sf_e)?.wrap_rc()
            )
                .wrap_ok()
        })
        .try_fold(vec![], |acc, x| {
            acc.chain_push(x?)
                .wrap_ok::<EvalErr>()
        });

    match struct_vec {
        Ok(vec) => Expr::Struct(type_annot.clone(), vec).wrap_ok(),
        Err(e) => e.wrap_err()
    }
}
