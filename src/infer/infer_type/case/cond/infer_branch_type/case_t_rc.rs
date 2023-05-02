use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::option::OptionAnyExt;
use crate::infra::triple::Triple;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::OptType;
use crate::parser::r#type::r#type::Type;

pub fn case_t_rc(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    then_expr_type: Type,
    constraint_acc: EnvRefConstraint,
    expect_type: &OptType,
    else_expr: &Expr
) -> InferTypeRet {
    // 当 expect_type 无类型时, 使用 then_expr_type hint
    let expect_type = match expect_type {
        Some(expect_type) =>
            if then_expr_type.can_lift_to(type_env, &expect_type) {
                expect_type.clone()
            } else {
                return TypeMissMatch::of_type(
                    &then_expr_type,
                    &expect_type
                )
                .into();
            },
        None => then_expr_type.clone()
    };

    let (else_expr_type, constraint_acc) = match else_expr
        .with_fallback_type(&expect_type)
        .infer_type(type_env, expr_env)?
    {
        Triple::L(t) => (t, constraint_acc),
        Triple::M(rc) =>
            match constraint_acc.extend_new(rc.constraint.clone()) {
                Some(constraint) => (rc.r#type, constraint),
                // 不可能发生的分支, 因为约束已被注入, 为保险而保留
                None =>
                    return TypeMissMatch::of_constraint(
                        &constraint_acc,
                        &rc.constraint
                    )
                    .into(),
            },

        Triple::R(ri) =>
            return ri.with_constraint_acc(constraint_acc),
    };

    InferTypeRet::from_auto_lift(
        type_env,
        &else_expr_type,
        &expect_type.some(),
        constraint_acc.some()
    )
}
