#[cfg(test)]
mod test;

use std::rc::Rc;

use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::{eval_expr, EvalRet};
use crate::eval::r#type::eval_err::EvalErr;
use crate::eval::r#type::expr::{Expr, StructField};
use crate::eval::r#type::r#type::Type;
use crate::infra::result::ResultAnyExt;
use crate::infra::vec::VecExt;

pub fn case_struct(
    type_env: &TypeEnv,
    expr_env: Rc<ExprEnv>,
    type_annot: &Type,
    struct_vec: &Vec<StructField>
) -> EvalRet {
    let struct_vec = struct_vec
        .into_iter()
        .map(|(sf_n, sf_t, sf_e)| {
            (
                sf_n.clone(),
                sf_t.clone(),
                eval_expr(type_env, expr_env.clone(), sf_e)?
            )
                .ok()
        })
        .try_fold(vec![], |acc, x| {
            acc.chain_push(x?)
                .ok::<EvalErr>()
        });

    match struct_vec {
        Ok(vec) => Expr::Struct(type_annot.clone(), vec).ok(),
        Err(e) => e.err()
    }
}
