use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::infer_type::r#fn::{
    with_constraint_lift_or_miss_match,
    with_constraint_unify_or_miss_match
};
use crate::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infer_type::r#type::GetTypeReturn;
use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::Type;

pub fn case_t_rc(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    then_expr_type: Type,
    constraint_acc: EnvRefConstraint,
    expect_type: &MaybeType,
    else_expr: &Expr
) -> GetTypeReturn {
    // 当 expect_type 无类型时, 使用 then_expr_type hint
    let expect_type = match expect_type {
        Some(expect_type) => expect_type.clone(),
        None => then_expr_type.clone()
    }
    .some();

    let (else_expr_type, constraint_acc) = match else_expr
        .try_with_fallback_type(&expect_type)
        .infer_type(type_env, expr_env)
    {
        Quad::L(t) => (t, constraint_acc),
        Quad::ML(rc) =>
            match constraint_acc.extend_new(rc.constraint.clone()) {
                Some(constraint) => (rc.r#type, constraint),
                None =>
                    return TypeMissMatch::of_constraint(
                        &constraint_acc,
                        &rc.constraint
                    )
                    .into(),
            },
        mr_r => return mr_r
    };

    match expect_type {
        Some(expect_type) => {
            match then_expr_type.lift_to(type_env, &expect_type) {
                None =>
                    return TypeMissMatch::of_type(
                        &then_expr_type,
                        &expect_type
                    )
                    .into(),
                _ => {}
            };
            with_constraint_lift_or_miss_match(
                constraint_acc,
                type_env,
                &else_expr_type,
                &expect_type
            )
        }
        _ => with_constraint_unify_or_miss_match(
            constraint_acc,
            type_env,
            &then_expr_type,
            &else_expr_type
        )
    }
}
