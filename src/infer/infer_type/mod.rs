mod case;
pub mod r#fn;
#[cfg(test)]
mod test;

pub mod r#type;

use std::rc::Rc;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::require_constraint::require_constraint;
use crate::infra::quad::Quad;
use crate::infra::triple::Triple;
use crate::parser::expr::r#type::Expr;

pub fn infer_type(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    expr: &Expr
) -> InferTypeRet {
    #[cfg(feature = "infer_log")]
    {
        let log =
            format!("{:8}{:>10} │ {expr:?}", "[infer]", "TypeOf");
        println!("{log}");
    }

    let result = match expr {
        Expr::Int(expect_type, i) =>
            case::int(type_env, expect_type, i),
        Expr::Unit(expect_type) => case::unit(type_env, expect_type),
        Expr::Discard(expect_type) =>
            case::discard(type_env, expect_type),

        Expr::EnvRef(expect_type, ref_name) =>
            case::env_ref(type_env, expr_env, expect_type, ref_name),

        Expr::Cond(expect_type, bool_expr, then_expr, else_expr) =>
            case::cond(
                type_env,
                expr_env,
                expect_type,
                bool_expr,
                then_expr,
                else_expr
            ),

        Expr::Closure(
            expect_type,
            input_name,
            input_type,
            output_expr
        ) => case::closure(
            type_env,
            expr_env,
            expect_type,
            input_name,
            input_type,
            output_expr
        ),

        Expr::Let(
            expect_type,
            rec_assign,
            assign_name,
            assign_type,
            assign_expr,
            scope_expr
        ) => case::r#let(
            type_env,
            expr_env,
            expect_type,
            rec_assign,
            assign_name,
            assign_type,
            assign_expr,
            scope_expr
        ),

        Expr::Struct(expect_type, vec) =>
            case::r#struct(type_env, expr_env, expect_type, vec),

        Expr::Apply(expect_type, lhs_expr, rhs_expr) => case::apply(
            type_env,
            expr_env,
            expect_type,
            lhs_expr,
            rhs_expr
        ),

        Expr::Match(expect_type, target_expr, vec) => case::r#match(
            type_env,
            expr_env,
            expect_type,
            target_expr,
            vec
        )
    };

    #[cfg(feature = "infer_log_min")]
    {
        let dbg_type = match result.clone() {
            Quad::L(x) =>
                format!("{:8}{:>10} │ {x:?}", "[infer]", "Inferred"),
            Quad::ML(x) =>
                format!("{:8}{:>10} │ {x:?}", "[infer]", "Inferred"),
            Quad::MR(x) =>
                format!("{:8}{:>10} │ {x:?}", "[infer]", "Inferred"),
            Quad::R(x) =>
                format!("{:8}{:>10} │ {x:?}", "[infer]", "Inferred"),
        };

        let log = if cfg!(feature = "infer_log") {
            let dbg_expr = format!(" of {expr:?}");
            format!("{dbg_type}{dbg_expr}")
        } else {
            dbg_type
        };

        println!("{log}");
    }

    match result {
        Quad::MR(ref ri) if !ri.constraint.is_empty() => {
            let constraint_acc = ri.constraint.clone();
            let new_expr_env = expr_env
                .extend_constraint_new(constraint_acc.clone());

            match infer_type(type_env, &new_expr_env, expr)? {
                Triple::L(typed_expr) =>
                    require_constraint(typed_expr, constraint_acc),
                Triple::M(rc) =>
                    rc.with_constraint_acc(constraint_acc),
                _ => result
            }
        }
        _ => result
    }
}
