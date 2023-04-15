mod case_ri;
mod case_t_rc;

use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::parser::expr::Expr;
use crate::type_checker::get_type::case::cond::case_ri::case_ri;
use crate::type_checker::get_type::case::cond::case_t_rc::case_t_rc;
use crate::type_checker::get_type::get_type_with_hint;
use crate::type_checker::get_type::r#type::GetTypeReturn;
use crate::unifier::can_lift;
use crate::{bool_type, empty_constraint, type_miss_match};

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &MaybeType,
    bool_expr: &Expr,
    then_expr: &Expr,
    else_expr: &Expr
) -> GetTypeReturn {
    let bool_expr_type = get_type_with_hint(
        type_env,
        expr_env,
        bool_expr,
        &bool_type!().some()
    );

    // bool_expr must be boolean types
    let constraint = match &bool_expr_type {
        Quad::L(bool_expr_type) =>
            if can_lift(type_env, &bool_expr_type, &bool_type!()) {
                empty_constraint!()
            } else {
                return type_miss_match!();
            },
        Quad::ML(rc) =>
            if can_lift(type_env, &rc.r#type, &bool_type!()) {
                rc.constraint.clone()
            } else {
                return type_miss_match!();
            },
        // 需要类型信息或者类型不匹配, 由于 Cond 没有环境注入, 不应处理这些情况
        mr_r => return mr_r.clone()
    };

    let then_expr_type = get_type_with_hint(
        type_env,
        expr_env,
        then_expr,
        expect_type
    );

    match then_expr_type {
        Quad::L(_) | Quad::ML(_) => case_t_rc(
            type_env,
            expr_env,
            then_expr_type,
            constraint,
            expect_type,
            else_expr
        ),
        Quad::MR(_)
            if then_expr.is_no_type_annotation() &&
                expect_type.is_none() =>
            return case_ri(
                type_env,
                expr_env,
                expect_type,
                bool_expr,
                else_expr,
                then_expr
            ),
        mr_r => return mr_r.clone()
    }
}
