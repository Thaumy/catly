use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::require_constraint::require_constraint;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::option::OptionAnyExt;
use crate::infra::r#box::BoxAnyExt;
use crate::infra::triple::Triple;
use crate::parser::expr::r#type::Expr;

pub fn case_ri(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    constraint_acc: EnvRefConstraint,
    bool_expr: &Expr,
    else_expr: &Expr,
    then_expr: &Expr
) -> InferTypeRet {
    let (else_expr_type, constraint_acc) = match else_expr
        .infer_type(type_env, expr_env)?
    {
        Triple::L(t) => (t, constraint_acc),
        // 需要收集这些作用于外层环境的约束并传播, 因为它们可能对推导 then_expr_type 有所帮助
        Triple::M(rc) =>
            match constraint_acc.extend_new(rc.constraint.clone()) {
                Some(erc) => (rc.r#type, erc),
                // 理论上无法抵达的分支, 因为 then_expr 的约束会被注入环境
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

    let cond_expr = Expr::Cond(
        else_expr_type.some(),
        bool_expr.clone().boxed(),
        then_expr.clone().boxed(),
        else_expr.clone().boxed()
    );

    let new_expr_env =
        expr_env.extend_constraint_new(constraint_acc.clone());

    match cond_expr.infer_type(type_env, &new_expr_env)? {
        Triple::L(t) => require_constraint(t, constraint_acc),
        Triple::M(rc) => rc.with_constraint_acc(constraint_acc),
        Triple::R(ri) => ri.with_constraint_acc(constraint_acc)
    }
}
