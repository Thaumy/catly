use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::require_constraint::require_constraint;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::option::OptionAnyExt;
use crate::infra::quad::Quad;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::OptType;
use crate::parser::r#type::r#type::Type;

pub fn case_t_rc(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    assign_expr_type: Type,
    constraint_acc: EnvRefConstraint,
    expect_type: &OptType,
    assign_name: &str,
    assign_type: &OptType,
    assign_expr: &Expr,
    scope_expr: &Expr
) -> InferTypeRet {
    // Lift assign_expr_type to assign_type
    let assign_type = match assign_expr_type
        .lift_to_or_left(type_env, assign_type)
    {
        None =>
            return TypeMissMatch::of_type(
                &assign_expr_type,
                &assign_type.clone().unwrap()
            )
            .into(),
        Some(t) => t
    };

    // Env inject
    let expr_env = expr_env.extend_new(
        assign_name.to_string(),
        assign_type.some(),
        assign_expr.clone().some()
    );

    // Hint scope_expr with expect_type and get scope_expr_type
    let scope_expr_type = scope_expr
        .with_opt_fallback_type(expect_type)
        .infer_type(type_env, &expr_env);

    match scope_expr_type {
        Quad::L(scope_expr_type) => match scope_expr_type
            .lift_to_or_left(type_env, expect_type)
        {
            Some(t) => require_constraint(t, constraint_acc),
            None => TypeMissMatch::of_type(
                &scope_expr_type,
                &expect_type.clone().unwrap()
            )
            .into()
        },
        // 由于 assign_type 存在, 所以此处的约束作用于外层环境, 传播之
        Quad::ML(rc) =>
            match constraint_acc.extend_new(rc.constraint.clone()) {
                Some(constraint) => InferTypeRet::from_auto_lift(
                    type_env,
                    &rc.r#type,
                    expect_type,
                    constraint.some()
                ),
                // 由于在获取 assign_type 时已将附加约束注入环境
                // 所以此处的约束冲突在理论上永远不会发生
                // 但是, 检查一下会更保险些
                None => TypeMissMatch::of_constraint(
                    &constraint_acc,
                    &rc.constraint
                )
                .into()
            },

        Quad::MR(ri) => ri.with_constraint_acc(constraint_acc),

        r => r
    }
}
