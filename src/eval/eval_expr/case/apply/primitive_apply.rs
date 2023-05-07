use std::ops::Rem;

use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::{eval_expr, EvalRet};
use crate::eval::r#macro::false_type;
use crate::eval::r#macro::namely_type;
use crate::eval::r#macro::true_type;
use crate::eval::r#type::eval_err::EvalErr;
use crate::eval::r#type::expr::primitive_op::PrimitiveOp;
use crate::eval::r#type::expr::Expr;
use crate::eval::r#type::r#type::Type;
use crate::infra::option::OptionAnyExt;
use crate::infra::result::ResultAnyExt;

fn eval_to_int(
    type_env: &TypeEnv,
    expr_env: Box<ExprEnv>,
    expr: &Expr
) -> Result<i64, EvalErr> {
    match eval_expr(type_env, expr_env, expr)? {
        Expr::Int(Type::NamelyType(n), i) if n == "Int" =>
            i.clone().ok(),
        _ => panic!("Impossible non-int expr: {expr:?}")
    }
}

fn eval_to_bool(
    type_env: &TypeEnv,
    expr_env: Box<ExprEnv>,
    expr: &Expr
) -> Result<bool, EvalErr> {
    match eval_expr(type_env, expr_env, expr)? {
        Expr::Int(Type::NamelyType(n), 1) if n == "True" => true.ok(),
        Expr::Int(Type::NamelyType(n), 0) if n == "False" =>
            false.ok(),
        _ => panic!("Impossible non-bool expr: {expr:?}")
    }
}

// TODO: refactor to sub mod
pub fn primitive_apply(
    type_env: &TypeEnv,
    expr_env: Box<ExprEnv>,
    primitive_op: &PrimitiveOp,
    rhs_expr: &Expr
) -> EvalRet {
    fn int_expr(i: i64) -> Expr { Expr::Int(namely_type!("Int"), i) }
    fn bool_expr(b: bool) -> Expr {
        match b {
            true => Expr::Int(true_type!(), 1),
            false => Expr::Int(false_type!(), 0)
        }
    }

    let lhs_int =
        |lhs_expr| eval_to_int(type_env, expr_env.clone(), lhs_expr);
    let rhs_int =
        || eval_to_int(type_env, expr_env.clone(), rhs_expr);

    let lhs_bool =
        |lhs_expr| eval_to_bool(type_env, expr_env.clone(), lhs_expr);
    let rhs_bool =
        || eval_to_bool(type_env, expr_env.clone(), rhs_expr);

    match primitive_op {
        // neg
        PrimitiveOp::Neg => int_expr(-rhs_int()?),
        // add
        PrimitiveOp::Add(None) =>
            PrimitiveOp::Add(rhs_expr.clone().some()).into(),
        PrimitiveOp::Add(Some(e)) =>
            int_expr(lhs_int(e)? + rhs_int()?),
        // sub
        PrimitiveOp::Sub(None) =>
            PrimitiveOp::Sub(rhs_expr.clone().some()).into(),
        PrimitiveOp::Sub(Some(e)) =>
            int_expr(lhs_int(e)? - rhs_int()?),
        // mul
        PrimitiveOp::Mul(None) =>
            PrimitiveOp::Mul(rhs_expr.clone().some()).into(),
        PrimitiveOp::Mul(Some(e)) =>
            int_expr(lhs_int(e)? * rhs_int()?),
        // mod
        PrimitiveOp::Mod(None) =>
            PrimitiveOp::Mod(rhs_expr.clone().some()).into(),
        PrimitiveOp::Mod(Some(e)) => {
            let u_l = lhs_int(e)? as u64;
            let u_r = rhs_int()? as u64;
            int_expr(u_l.rem_euclid(u_r) as i64)
        }
        // rem
        PrimitiveOp::Rem(None) =>
            PrimitiveOp::Rem(rhs_expr.clone().some()).into(),
        PrimitiveOp::Rem(Some(e)) =>
            int_expr(lhs_int(e)?.rem(rhs_int()?)),

        // gt
        PrimitiveOp::Gt(None) =>
            PrimitiveOp::Gt(rhs_expr.clone().some()).into(),
        PrimitiveOp::Gt(Some(e)) =>
            bool_expr(lhs_int(e)? > rhs_int()?),
        // eq
        PrimitiveOp::Eq(None) =>
            PrimitiveOp::Eq(rhs_expr.clone().some()).into(),
        PrimitiveOp::Eq(Some(e)) =>
            bool_expr(lhs_int(e)? == rhs_int()?),
        // lt
        PrimitiveOp::Lt(None) =>
            PrimitiveOp::Lt(rhs_expr.clone().some()).into(),
        PrimitiveOp::Lt(Some(e)) =>
            bool_expr(lhs_int(e)? < rhs_int()?),

        // not
        PrimitiveOp::Not => bool_expr(!rhs_bool()?),
        // and
        PrimitiveOp::And(None) =>
            PrimitiveOp::And(rhs_expr.clone().some()).into(),
        PrimitiveOp::And(Some(e)) =>
            bool_expr(lhs_bool(e)? && rhs_bool()?),
        // or
        PrimitiveOp::Or(None) =>
            PrimitiveOp::Or(rhs_expr.clone().some()).into(),
        PrimitiveOp::Or(Some(e)) =>
            bool_expr(lhs_bool(e)? || rhs_bool()?),
    }
    .ok()
}
