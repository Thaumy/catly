pub mod case;
pub mod r#fn;
#[cfg(test)]
mod test;
pub mod r#type;

use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer_type::r#type::require_constraint::require_constraint;
use crate::infra::quad::Quad;
use crate::parser::expr::r#type::Expr;

pub fn infer_type(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expr: &Expr
) -> InferTypeRet {
    println!("{:8}{:>10} │ {expr:?}", "[infer]", "TypeOf");

    let result = match expr {
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
    };

    let log = match result.clone() {
        Quad::L(x) => format!(
            "{:8}{:>10} │ {x:?} of {expr:?}",
            "[infer]", "Inferred"
        ),
        Quad::ML(x) => format!(
            "{:8}{:>10} │ {x:?} of {expr:?}",
            "[infer]", "Inferred"
        ),
        Quad::MR(x) => format!(
            "{:8}{:>10} │ {x:?} of {expr:?}",
            "[infer]", "Inferred"
        ),
        Quad::R(x) => format!(
            "{:8}{:>10} │ {x:?} of {expr:?}",
            "[infer]", "Inferred"
        )
    };

    println!("{log}");

    match &result {
        Quad::MR(ri) if !ri.constraint.is_empty() => {
            let constraint_acc = ri.constraint.clone();
            let expr_env = expr_env
                .extend_constraint_new(constraint_acc.clone());
            match infer_type(type_env, &expr_env, expr) {
                Quad::L(t) => require_constraint(t, constraint_acc),
                Quad::ML(rc) =>
                    rc.with_constraint_acc(constraint_acc),
                _ => result
            }
        }
        _ => result
    }
}
