mod case;
#[cfg(test)]
mod test;

use std::rc::Rc;

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
    expr_env: &Rc<ExprEnv>,
    expr: &Rc<Expr>
) -> EvalRet {
    #[cfg(feature = "eval_log")]
    {
        let log = format!("{:8}{:>10} │ {expr:?}", "[eval]", "ValOf");
        println!("{log}");
    }

    let result = match expr.as_ref() {
        Expr::Discard(t) => case_discard(t),
        Expr::Int(t, i) => case_int(t, i),
        Expr::Unit(t) => case_unit(t),
        Expr::EnvRef(_, r_n) => case_env_ref(type_env, expr_env, r_n),
        // Closure 在如果被求值, 那么它一定是首次被求值(因为求值是惰性的), 所以一定未捕获环境
        // 此时它将捕获当前环境作为求值环境, 接着会立即被 Apply 消费
        Expr::Closure(t, i_n, i_t, o_e, None) =>
            case_closure(t, i_n, i_t, o_e, expr_env),
        // 不可能发生的情况, 不存在捕获环境的滞留 Closure, 因为求值是惰性的
        Expr::Closure(.., Some(_)) => unreachable!(),

        Expr::Struct(t, s_v) =>
            case_struct(type_env, expr_env, t, s_v),

        // PrimitiveOp 是一类特殊的 Closure
        // 它通过 Apply 的形式捕获环境, 所以在此处不进行环境捕获
        Expr::PrimitiveOp(t, op, None) => case_primitive_op(t, op),
        // PrimitiveOp 总是通过 Apply 捕获环境
        // 由于任何需要返回 PrimitiveOp 的表达式都需要对 PrimitiveOp 求值
        // 而此时 PrimitiveOp 一定不会捕获环境, 因为 Apply 还没有发生
        // 所以下面的情况不可能发生
        Expr::PrimitiveOp(.., Some(_)) => unreachable!(),

        Expr::Cond(b_e, t_e, f_e) =>
            case_cond(type_env, expr_env, b_e, t_e, f_e),
        Expr::Match(t_e, c_v) =>
            case_match(type_env, expr_env, t_e, c_v),
        Expr::Apply(l_e, r_e) =>
            case_apply(type_env, expr_env, l_e, r_e),
        Expr::Let(a_n, r_a, a_t, a_e, s_e) =>
            case_let(type_env, expr_env, r_a, a_n, a_t, a_e, s_e),
    };

    #[cfg(feature = "eval_log_min")]
    {
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
