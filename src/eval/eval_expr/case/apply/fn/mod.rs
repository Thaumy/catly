use std::rc::Rc;

use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::r#macro::{false_type, namely_type, true_type};
use crate::eval::r#type::eval_err::EvalErr;
use crate::eval::r#type::expr::Expr;
use crate::eval::r#type::r#type::Type;
use crate::infra::result::ResultAnyExt;

pub mod primitive_apply;
pub mod source_lhs_to_closure;

pub fn eval_to_bool(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    expr: &Rc<Expr>
) -> Result<bool, EvalErr> {
    match eval_expr(type_env, expr_env, expr)? {
        Expr::Int(Type::NamelyType(n), 1) if n == "True" => true.ok(),
        Expr::Int(Type::NamelyType(n), 0) if n == "False" =>
            false.ok(),
        _ => panic!("Impossible non-bool expr: {expr:?}")
    }
}

pub fn eval_to_int(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    expr: &Rc<Expr>
) -> Result<i64, EvalErr> {
    match eval_expr(type_env, expr_env, expr)? {
        Expr::Int(Type::NamelyType(n), i) if n == "Int" => i.ok(),
        _ => panic!("Impossible non-int expr: {expr:?}")
    }
}

#[inline]
fn int_expr(i: i64) -> Expr { Expr::Int(namely_type!("Int"), i) }

#[inline]
fn bool_expr(b: bool) -> Expr {
    match b {
        true => Expr::Int(true_type!(), 1),
        false => Expr::Int(false_type!(), 0)
    }
}
