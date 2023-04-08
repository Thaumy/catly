pub mod case;
pub mod r#fn;
pub mod r#macro;
pub mod r#type;

use crate::infra::alias::MaybeType;
use crate::parser::expr::Expr;
use crate::type_checker::get_type::r#type::{
    ExprEnv,
    GetTypeReturn,
    TypeEnv
};

pub fn get_type_with_hint(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expr: &Expr,
    hint: &MaybeType
) -> GetTypeReturn {
    let expr = expr
        .clone()
        .try_with_fallback_type(hint);

    get_type(type_env, expr_env, &expr)
}

pub fn get_type(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expr: &Expr
) -> GetTypeReturn {
    use crate::type_checker::get_type;
    match expr {
        Expr::Int(t, _) => {
            use case::int::case;
            case(type_env, t)
        }
        Expr::Unit(t) => {
            use case::unit::case;
            case(type_env, t)
        }
        Expr::Discard(t) => {
            use case::discard::case;
            case(type_env, t)
        }

        Expr::EnvRef(t, ref_name) => {
            use case::env_ref::case;
            case(type_env, expr_env, t, ref_name)
        }

        // 推导提示
        Expr::Cond(t, bool_expr, then_expr, else_expr) => {
            use case::cond::case;
            case(
                type_env, expr_env, t, bool_expr, then_expr,
                else_expr
            )
        }

        // 推导提示 + 类型解构 + 反向约束 + 约束消减
        Expr::Closure(t, input_name, input_type, output_expr) => {
            use case::closure::case;
            case(
                type_env,
                expr_env,
                t,
                input_name,
                input_type,
                output_expr
            )
        }

        // 推导提示 + 反向约束 + 约束消减 + 约束传播
        Expr::Let(
            t,
            assign_name,
            assign_type,
            assign_expr,
            scope_expr
        ) => {
            use case::r#let::case;
            case(
                type_env,
                expr_env,
                t,
                assign_name,
                assign_type,
                assign_expr,
                scope_expr
            )
        }

        // 推导提示 + 类型解构 + 反向约束
        Expr::Struct(t, vec) => {
            use case::r#struct::case;
            case(type_env, expr_env, t, vec)
        }

        Expr::Apply(t, lhs, rhs) => {
            use case::apply::case;
            case(t, lhs, rhs)
        }

        // 推导提示 + 反向约束 + 旁路推导 + 穿透推导
        Expr::Match(t, match_expr, vec) => {
            use case::r#match::case;
            case(t, match_expr, vec)
        }
    }
}
