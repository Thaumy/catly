use std::rc::Rc;

use crate::eval::env::ExprEnv;
use crate::eval::env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::EvalErr;
use crate::eval::Expr;
use crate::eval::Type;
use crate::eval::{false_type, namely_type, true_type};
use crate::infra::result::WrapResult;

mod primitive_apply;
mod source_lhs_to_closure;

pub use primitive_apply::*;
pub use source_lhs_to_closure::*;

pub fn eval_to_bool(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    expr: &Rc<Expr>
) -> Result<bool, EvalErr> {
    match eval_expr(type_env, expr_env, expr)? {
        Expr::Int(Type::NamelyType(n), 1) if n == "True" =>
            true.wrap_ok(),
        Expr::Int(Type::NamelyType(n), 0) if n == "False" =>
            false.wrap_ok(),
        _ => unreachable!()
    }
}

pub fn eval_to_int(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    expr: &Rc<Expr>
) -> Result<i64, EvalErr> {
    match eval_expr(type_env, expr_env, expr)? {
        Expr::Int(Type::NamelyType(n), i) if n == "Int" =>
            i.wrap_ok(),
        _ => unreachable!()
    }
}

#[inline]
pub fn int_expr(i: i64) -> Expr { Expr::Int(namely_type!("Int"), i) }

#[inline]
pub fn bool_expr(b: bool) -> Expr {
    match b {
        true => Expr::Int(true_type!(), 1),
        false => Expr::Int(false_type!(), 0)
    }
}
