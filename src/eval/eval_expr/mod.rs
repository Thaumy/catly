mod case;
#[cfg(test)]
mod test;

use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::case::apply::case_apply;
use crate::eval::eval_expr::case::closure::case_closure;
use crate::eval::eval_expr::case::cond::case_cond;
use crate::eval::eval_expr::case::discard::case_discard;
use crate::eval::eval_expr::case::env_ref::case_env_ref;
use crate::eval::eval_expr::case::int::case_int;
use crate::eval::eval_expr::case::primitive_op::case_primitive_op;
use crate::eval::eval_expr::case::r#let::case_let;
use crate::eval::eval_expr::case::r#match::case_match;
use crate::eval::eval_expr::case::r#struct::case_struct;
use crate::eval::eval_expr::case::unit::case_unit;
use crate::eval::r#type::eval_err::EvalErr;
use crate::eval::r#type::expr::Expr;

pub type EvalRet = Result<Expr, EvalErr>;

pub fn eval_expr(
    type_env: &TypeEnv,
    expr_env: Box<ExprEnv>,
    expr: &Expr
) -> EvalRet {
    if cfg!(feature = "eval_log") {
        let log = format!("{:8}{:>10} │ {expr:?}", "[eval]", "ValOf");
        println!("{log}");
    }

    let result = match expr {
        Expr::Discard(t) => case_discard(t),
        Expr::Int(t, i) => case_int(t.clone(), i.clone()),
        Expr::Unit(t) => case_unit(t.clone()),
        Expr::EnvRef(_, r_n) => case_env_ref(type_env, expr_env, r_n),
        Expr::Closure(t, i_n, i_t, o_e, env) =>
            case_closure(expr_env, t, i_n, i_t, o_e, env),
        Expr::Struct(t, s_v) =>
            case_struct(type_env, expr_env, t, s_v),

        Expr::PrimitiveOp(t, op, env) =>
            case_primitive_op(expr_env, t, op, env),

        Expr::Cond(b_e, t_e, f_e) =>
            case_cond(type_env, expr_env, b_e, t_e, f_e),
        Expr::Match(t_e, c_v) =>
            case_match(type_env, expr_env, t_e, c_v),
        Expr::Apply(l_e, r_e) =>
            case_apply(type_env, expr_env, l_e, r_e),
        Expr::Let(a_n, a_t, a_e, s_e) =>
            case_let(type_env, expr_env, a_n, a_t, a_e, s_e),
    };

    if cfg!(feature = "eval_log_min") {
        let dbg_type = match result.clone() {
            Ok(expr) => format!(
                "{:8}{:>10} │ {expr:?}",
                "[eval]", "Evaluated"
            ),
            Err(err) =>
                format!("{:8}{:>10} │ {err:?}", "[eval]", "Evaluated"),
        };

        let log = if cfg!(feature = "eval_log") {
            let dbg_expr = format!(" of {expr:?}");
            format!("{dbg_type}{dbg_expr}")
        } else {
            dbg_type
        };

        println!("{log}");
    }

    result
}
