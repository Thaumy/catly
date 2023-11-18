use std::ops::Rem;
use std::rc::Rc;

use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::case::apply::r#fn::{
    bool_expr,
    eval_to_bool,
    eval_to_int,
    int_expr
};
use crate::eval::eval_expr::EvalRet;
use crate::eval::r#type::expr::primitive_op::PrimitiveOp;
use crate::eval::r#type::expr::Expr;
use crate::infra::option::WrapOption;
use crate::infra::result::WrapResult;

pub fn primitive_apply(
    type_env: &TypeEnv,
    lhs_eval_env: &Rc<ExprEnv>,
    rhs_eval_env: &Rc<ExprEnv>,
    primitive_op: &PrimitiveOp,
    rhs_expr: &Rc<Expr>
) -> EvalRet {
    let lhs_int =
        |lhs_expr| eval_to_int(type_env, lhs_eval_env, lhs_expr);
    let lhs_bool =
        |lhs_expr| eval_to_bool(type_env, lhs_eval_env, lhs_expr);

    let rhs_int = || eval_to_int(type_env, rhs_eval_env, rhs_expr);
    let rhs_bool = || eval_to_bool(type_env, rhs_eval_env, rhs_expr);

    match primitive_op {
        // neg
        PrimitiveOp::Neg => int_expr(-rhs_int()?),
        // add
        PrimitiveOp::Add(None) =>
            PrimitiveOp::Add(rhs_expr.clone().wrap_some()).into(),
        PrimitiveOp::Add(Some(e)) =>
            int_expr(lhs_int(e)? + rhs_int()?),
        // sub
        PrimitiveOp::Sub(None) =>
            PrimitiveOp::Sub(rhs_expr.clone().wrap_some()).into(),
        PrimitiveOp::Sub(Some(e)) =>
            int_expr(lhs_int(e)? - rhs_int()?),
        // mul
        PrimitiveOp::Mul(None) =>
            PrimitiveOp::Mul(rhs_expr.clone().wrap_some()).into(),
        PrimitiveOp::Mul(Some(e)) =>
            int_expr(lhs_int(e)? * rhs_int()?),
        // div
        PrimitiveOp::Div(None) =>
            PrimitiveOp::Mul(rhs_expr.clone().wrap_some()).into(),
        PrimitiveOp::Div(Some(e)) =>
            int_expr(lhs_int(e)? / rhs_int()?),
        // mod
        PrimitiveOp::Mod(None) =>
            PrimitiveOp::Mod(rhs_expr.clone().wrap_some()).into(),
        PrimitiveOp::Mod(Some(e)) => {
            let u_l = lhs_int(e)? as u64;
            let u_r = rhs_int()? as u64;
            int_expr(u_l.rem_euclid(u_r) as i64)
        }
        // rem
        PrimitiveOp::Rem(None) =>
            PrimitiveOp::Rem(rhs_expr.clone().wrap_some()).into(),
        PrimitiveOp::Rem(Some(e)) =>
            int_expr(lhs_int(e)?.rem(rhs_int()?)),

        // gt
        PrimitiveOp::Gt(None) =>
            PrimitiveOp::Gt(rhs_expr.clone().wrap_some()).into(),
        PrimitiveOp::Gt(Some(e)) =>
            bool_expr(lhs_int(e)? > rhs_int()?),
        // eq
        PrimitiveOp::Eq(None) =>
            PrimitiveOp::Eq(rhs_expr.clone().wrap_some()).into(),
        PrimitiveOp::Eq(Some(e)) =>
            bool_expr(lhs_int(e)? == rhs_int()?),
        // lt
        PrimitiveOp::Lt(None) =>
            PrimitiveOp::Lt(rhs_expr.clone().wrap_some()).into(),
        PrimitiveOp::Lt(Some(e)) =>
            bool_expr(lhs_int(e)? < rhs_int()?),

        // not
        PrimitiveOp::Not => bool_expr(!rhs_bool()?),
        // and
        PrimitiveOp::And(None) =>
            PrimitiveOp::And(rhs_expr.clone().wrap_some()).into(),
        PrimitiveOp::And(Some(e)) =>
            bool_expr(lhs_bool(e)? && rhs_bool()?),
        // or
        PrimitiveOp::Or(None) =>
            PrimitiveOp::Or(rhs_expr.clone().wrap_some()).into(),
        PrimitiveOp::Or(Some(e)) =>
            bool_expr(lhs_bool(e)? || rhs_bool()?),
    }
    .wrap_ok()
}
