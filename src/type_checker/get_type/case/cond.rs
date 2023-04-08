use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::parser::expr::Expr;
use crate::type_checker::get_type::get_type_with_hint;
use crate::type_checker::get_type::r#fn::of_boolean_types;
use crate::type_checker::get_type::r#type::{
    ExprEnv,
    GetTypeReturn,
    TypeEnv
};
use crate::unifier::{lift, unify};
use crate::{
    bool_type,
    has_type,
    require_constraint,
    type_miss_match
};

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &MaybeType,
    bool_expr: &Expr,
    then_expr: &Expr,
    else_expr: &Expr
) -> GetTypeReturn {
    // TODO: Lazy init
    let mut constraint = vec![];

    let bool_expr_type = get_type_with_hint(
        type_env,
        expr_env,
        bool_expr,
        &bool_type!().some()
    );

    // bool_expr must be boolean types
    match &bool_expr_type {
        Quad::L(t) =>
            if !of_boolean_types(&t) {
                return type_miss_match!();
            },
        Quad::ML(rc) => {
            if !of_boolean_types(&rc.r#type) {
                return type_miss_match!();
            }
            constraint.append(&mut rc.constraint.clone())
        }
        // 需要类型信息或者类型不匹配, 由于 Cond 没有环境注入, 不应处理这些情况
        mr_r => return mr_r.clone()
    };

    let then_expr_type = match get_type_with_hint(
        type_env,
        expr_env,
        then_expr,
        expect_type
    ) {
        Quad::L(t) => t,
        Quad::ML(rc) => {
            constraint.append(&mut rc.constraint.clone());
            rc.r#type
        }
        mr_r => return mr_r.clone()
    };
    let else_expr_type = match get_type_with_hint(
        type_env,
        expr_env,
        else_expr,
        expect_type
    ) {
        Quad::L(t) => t,
        Quad::ML(rc) => {
            constraint.append(&mut rc.constraint.clone());
            rc.r#type
        }
        mr_r => return mr_r.clone()
    };

    let t = match match expect_type {
        Some(t) => lift(type_env, &then_expr_type, t)
            .and_then(|t| lift(type_env, &else_expr_type, &t)),
        _ => unify(type_env, &then_expr_type, &else_expr_type)
    } {
        Some(t) => t,
        // 提升或合一失败, 类型不匹配
        _ => return type_miss_match!()
    };

    if constraint.is_empty() {
        has_type!(t)
    } else {
        require_constraint!(t, constraint)
    }
}
