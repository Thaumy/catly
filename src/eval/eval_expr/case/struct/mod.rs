#[cfg(test)]
mod test;

use std::rc::Rc;

use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::{eval_expr, EvalRet};
use crate::eval::r#type::eval_err::EvalErr;
use crate::eval::r#type::expr::{Expr, StructField};
use crate::eval::r#type::r#type::Type;
use crate::infra::rc::RcAnyExt;
use crate::infra::result::WrapResult;
use crate::infra::vec::VecExt;

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
                eval_expr(type_env, expr_env, sf_e)?.rc()
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
