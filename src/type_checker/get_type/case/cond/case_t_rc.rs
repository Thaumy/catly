use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::parser::expr::Expr;
use crate::type_checker::get_type::get_type_with_hint;
use crate::type_checker::get_type::r#type::{
    EnvRefConstraint,
    GetTypeReturn
};
use crate::unifier::{lift, unify};
use crate::{has_type, require_constraint, type_miss_match};

pub fn case_t_rc(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    then_expr_type: GetTypeReturn,
    constraint: EnvRefConstraint,
    expect_type: &MaybeType,
    else_expr: &Expr
) -> GetTypeReturn {
    let (then_expr_type, constraint) = match then_expr_type {
        Quad::L(then_expr_type) => (then_expr_type, constraint),
        Quad::ML(rc) => match constraint.extend_new(rc.constraint) {
            Some(constraint) => (rc.r#type, constraint),
            None => return type_miss_match!()
        },
        _ => panic!("Impossible then_expr_type: {:?}", then_expr_type)
    };

    // 当 expect_type 无类型时, 使用 then_expr_type hint
    let expect_type = match expect_type {
        Some(expect_type) => expect_type.clone(),
        None => then_expr_type.clone()
    }
    .some();

    let (else_expr_type, constraint) = match get_type_with_hint(
        type_env,
        expr_env,
        else_expr,
        &expect_type
    ) {
        Quad::L(t) => (t, constraint),
        Quad::ML(rc) => match constraint.extend_new(rc.constraint) {
            Some(constraint) => (rc.r#type, constraint),
            None => return type_miss_match!()
        },
        mr_r => return mr_r.clone()
    };

    let t = match match expect_type {
        Some(t) => lift(type_env, &then_expr_type, &t)
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
