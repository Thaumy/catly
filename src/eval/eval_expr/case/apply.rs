use std::ops::{Deref, Rem};

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
use crate::infra::either::{AnyExt, Either};
use crate::infra::option::AnyExt as OptAnyExt;
use crate::infra::result::AnyExt as ResAnyExt;

pub fn source_env_ref<'t>(
    expr_env: &'t ExprEnv,
    expr: &Expr
) -> Either<(Option<String>, Type, Expr, ExprEnv<'t>), PrimitiveOp> {
    match expr {
        Expr::EnvRef(_, ref_name) => {
            // TODO:
            // 此处为逐层查找 env_ref
            // 可以设置穿透的访问链, 提高 env_ref 的检索效率
            let (src_expr, src_env) = expr_env
                .get_expr_and_env(ref_name.as_str())
                .unwrap_or_else(|| {
                    panic!(
                        "EnvRef {ref_name:?} not found in expr env"
                    )
                });

            source_env_ref(src_env, src_expr)
        }
        Expr::Closure(_, input_name, input_type, output_expr) => (
            input_name.clone(),
            input_type.clone(),
            *output_expr.clone(),
            expr_env.clone()
        )
            .l(),
        Expr::PrimitiveOp(_, op) => op.deref().clone().r(),
        _ => panic!("Impossible expr: {expr:?}")
    }
}

fn eval_to_int(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
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
    expr_env: &ExprEnv,
    expr: &Expr
) -> Result<bool, EvalErr> {
    match eval_expr(type_env, expr_env, expr)? {
        Expr::Int(Type::NamelyType(n), 1) if n == "True" => true.ok(),
        Expr::Int(Type::NamelyType(n), 0) if n == "False" =>
            false.ok(),
        _ => panic!("Impossible non-bool expr: {expr:?}")
    }
}

pub fn primitive_apply(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
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
        |lhs_expr| eval_to_int(type_env, expr_env, lhs_expr);
    let rhs_int = || eval_to_int(type_env, expr_env, rhs_expr);

    let lhs_bool =
        |lhs_expr| eval_to_bool(type_env, expr_env, lhs_expr);
    let rhs_bool = || eval_to_bool(type_env, expr_env, rhs_expr);

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

pub fn case_apply(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    lhs_expr: &Expr,
    rhs_expr: &Expr
) -> EvalRet {
    match source_env_ref(expr_env, lhs_expr) {
        Either::L((
            input_name,
            input_type,
            output_expr,
            output_eval_env
        )) => {
            let extended_eval_env = match input_name {
                Some(input_name) => output_eval_env.extend_new(
                    input_name,
                    input_type,
                    rhs_expr.clone(),
                    expr_env.clone()
                ),
                None => output_eval_env
            };

            eval_expr(type_env, &extended_eval_env, &output_expr)
        }
        Either::R(primitive_op) => primitive_apply(
            type_env,
            expr_env,
            &primitive_op,
            rhs_expr
        )
    }
}
