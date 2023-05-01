mod infer_branch_type;
#[cfg(test)]
mod test;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::r#macro::bool_type;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::case::cond::infer_branch_type::infer_branch_type;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::quad::{Quad, QuadAnyExt};
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::OptType;

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &OptType,
    bool_expr: &Expr,
    then_expr: &Expr,
    else_expr: &Expr
) -> InferTypeRet {
    let bool_expr_type = bool_expr
        .with_fallback_type(&bool_type!())
        .infer_type(type_env, expr_env);

    // bool_expr must be boolean types
    let constraint_acc = match &bool_expr_type {
        Quad::L(bool_expr_type) =>
            if bool_expr_type.can_lift_to(type_env, &bool_type!()) {
                EnvRefConstraint::empty()
            } else {
                return TypeMissMatch::of_type(
                    bool_expr_type,
                    &bool_type!()
                )
                .into();
            },
        Quad::ML(rc) =>
            if rc
                .r#type
                .can_lift_to(type_env, &bool_type!())
            {
                rc.constraint.clone()
            } else {
                return TypeMissMatch::of_type(
                    &rc.r#type,
                    &bool_type!()
                )
                .into();
            },
        // 求取分支类型, 因为分支约束可能有助于求得 bool_expr 类型
        // 约束将在下一轮次被注入环境, 同时也会再次求 bool_expr 类型
        Quad::MR(ri) => {
            let constraint_acc = ri.constraint.clone();
            let new_expr_env = &expr_env
                .extend_constraint_new(constraint_acc.clone());

            return match infer_branch_type(
                type_env,
                new_expr_env,
                expect_type,
                bool_expr,
                constraint_acc,
                then_expr,
                else_expr
            ) {
                // infer_branch_type 会自动管理累积约束, 无需再次收集扩展
                x @ Quad::ML(_) | x @ Quad::MR(_) => x,
                // 未产生约束, 返回原错误
                _ => ri.clone().quad_mr()
            };
        }
        // 类型不匹配
        r => return r.clone()
    };

    // 由于求 bool_expr_type 产生的约束可能对接下来有帮助, 所以需要注入到环境
    let new_expr_env =
        &expr_env.extend_constraint_new(constraint_acc.clone());

    infer_branch_type(
        type_env,
        new_expr_env,
        expect_type,
        bool_expr,
        constraint_acc,
        then_expr,
        else_expr
    )
}
