pub mod case;
pub mod r#fn;
pub mod r#macro;
#[cfg(test)]
mod test;
pub mod r#type;

use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::get_type::r#type::GetTypeReturn;
use crate::infra::alias::MaybeType;
use crate::parser::expr::r#type::Expr;

pub fn get_type_with_hint(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expr: &Expr,
    hint: &MaybeType // TODO: 确定 hint 与 expect 语义
) -> GetTypeReturn {
    let expr = expr.try_with_fallback_type(hint);

    get_type(type_env, expr_env, &expr)
}

pub fn get_type(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expr: &Expr
) -> GetTypeReturn {
    println!("{:8}{:>10} │ {expr:?}", "[infer]", "GetType");

    match expr {
        Expr::Int(expect_type, _) => {
            use case::int::case;
            case(type_env, expect_type)
        }
        Expr::Unit(expect_type) => {
            use case::unit::case;
            case(type_env, expect_type)
        }
        Expr::Discard(expect_type) => {
            use case::discard::case;
            case(type_env, expect_type)
        }

        Expr::EnvRef(expect_type, ref_name) => {
            use case::env_ref::case;
            case(type_env, expr_env, expect_type, ref_name)
        }

        // 推导提示
        Expr::Cond(expect_type, bool_expr, then_expr, else_expr) => {
            use case::cond::case;
            case(
                type_env,
                expr_env,
                expect_type,
                bool_expr,
                then_expr,
                else_expr
            )
        }

        Expr::Closure(
            expect_type,
            input_name,
            input_type,
            output_expr
        ) => {
            use case::closure::case;
            case(
                type_env,
                expr_env,
                expect_type,
                input_name,
                input_type,
                output_expr
            )
        }

        Expr::Let(
            expect_type,
            assign_name,
            assign_type,
            assign_expr,
            scope_expr
        ) => {
            use case::r#let::case;
            case(
                type_env,
                expr_env,
                expect_type,
                assign_name,
                assign_type,
                assign_expr,
                scope_expr
            )
        }

        Expr::Struct(expect_type, vec) => {
            use case::r#struct::case;
            case(type_env, expr_env, expect_type, vec)
        }

        Expr::Apply(expect_type, lhs_expr, rhs_expr) => {
            use case::apply::case;
            case(type_env, expr_env, expect_type, lhs_expr, rhs_expr)
        }

        Expr::Match(expect_type, target_expr, vec) => {
            use case::r#match::case;
            case(type_env, expr_env, expect_type, target_expr, vec)
        }
    }
}
