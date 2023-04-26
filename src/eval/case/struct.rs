use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::r#type::expr::{Expr, StructField};
use crate::eval::r#type::r#type::Type;
use crate::eval::{eval, EvalRet};
use crate::infra::result::AnyExt;
use crate::infra::vec::Ext;

pub fn case_struct(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    type_annot: &Type,
    struct_vec: &Vec<StructField>
) -> EvalRet {
    let struct_vec = struct_vec
        .iter()
        .map(|(sf_n, sf_t, sf_e)| {
            match eval(type_env, expr_env, sf_e) {
                Ok(sf_e) => (sf_n.clone(), sf_t.clone(), sf_e).ok(),
                Err(e) => e.err()
            }
        })
        .try_fold(vec![], |acc, x| match x {
            Ok(sf) => acc.chain_push(sf).ok(),
            Err(e) => e.err()
        });

    match struct_vec {
        Ok(vec) => Expr::Struct(type_annot.clone(), vec).ok(),
        Err(e) => e.err()
    }
}
