mod case_ri;
mod case_t_rc;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::case::cond::infer_branch_type::case_ri::case_ri;
use crate::infer::infer_type::case::cond::infer_branch_type::case_t_rc::case_t_rc;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::quad::Quad;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::OptType;

pub fn infer_branch_type(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &OptType,
    bool_expr: &Expr,
    constraint_acc: EnvRefConstraint,
    then_expr: &Expr,
    else_expr: &Expr
) -> InferTypeRet {
    let then_expr_type = then_expr
        .with_opt_fallback_type(expect_type)
        .infer_type(type_env, expr_env);

    match then_expr_type {
        Quad::L(_) | Quad::ML(_) => {
            let (then_expr_type, constraint) =
                then_expr_type.unwrap_type_and_constraint();
            let constraint_acc =
                match constraint_acc.extend_new(constraint.clone()) {
                    Some(constraint) => constraint,
                    None =>
                        return TypeMissMatch::of_constraint(
                            &constraint_acc,
                            &constraint
                        )
                        .into(),
                };

            // 与上同理
            let new_expr_env = &expr_env
                .extend_constraint_new(constraint_acc.clone());

            case_t_rc(
                type_env,
                new_expr_env,
                then_expr_type,
                constraint_acc,
                expect_type,
                else_expr
            )
        }

        Quad::MR(ri)
            if then_expr.is_no_type_annot() &&
                expect_type.is_none() =>
        {
            let new_expr_env = &expr_env
                .extend_constraint_new(ri.constraint.clone());

            case_ri(
                type_env,
                new_expr_env,
                ri.constraint,
                bool_expr,
                else_expr,
                then_expr
            )
            .0
        }

        Quad::MR(ri) => ri.with_constraint_acc(constraint_acc),

        r => r
    }
}
