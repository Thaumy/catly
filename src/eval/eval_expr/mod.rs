mod case;

use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::case::apply::case_apply;
use crate::eval::eval_expr::case::cond::case_cond;
use crate::eval::eval_expr::case::discard::case_discard;
use crate::eval::eval_expr::case::env_ref::case_env_ref;
use crate::eval::eval_expr::case::int::case_int;
use crate::eval::eval_expr::case::r#let::case_let;
use crate::eval::eval_expr::case::r#match::case_match;
use crate::eval::eval_expr::case::r#struct::case_struct;
use crate::eval::eval_expr::case::unit::case_unit;
use crate::eval::r#type::eval_err::EvalErr;
use crate::eval::r#type::expr::Expr;
use crate::infra::result::AnyExt;

pub type EvalRet = Result<Expr, EvalErr>;

pub fn eval_expr(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expr: &Expr
) -> EvalRet {
    if cfg!(feature = "rt_log") {
        let log = format!("{:8}{:>10} │ {expr:?}", "[rt]", "ValOf");
        println!("{log}");
    }

    match expr {
        Expr::Discard(t) => case_discard(t),
        Expr::Int(t, i) => case_int(t.clone(), i.clone()),
        Expr::Unit(t) => case_unit(t.clone()),
        Expr::EnvRef(_, r_n) => case_env_ref(type_env, expr_env, r_n),
        Expr::Apply(_, l_e, r_e) =>
            case_apply(type_env, expr_env, l_e, r_e),
        Expr::Cond(_, b_e, t_e, f_e) =>
            case_cond(type_env, expr_env, b_e, t_e, f_e),
        Expr::Closure(..) => expr.clone().ok(), // 直接返回 closure, 环境将由求取该值的 case 保留
        Expr::Struct(t, s_v) =>
            case_struct(type_env, expr_env, t, s_v),
        Expr::Match(_, t_e, c_v) =>
            case_match(type_env, expr_env, t_e, c_v),
        Expr::Let(_, a_n, a_t, a_e, s_e) =>
            case_let(type_env, expr_env, a_n, a_t, a_e, s_e),

        primitive_op => primitive_op.clone().ok()
    }
}
