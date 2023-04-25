use std::assert_matches::assert_matches;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#fn::has_type;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::require_info::RequireInfo;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infer::infer_type::test::parse_env;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::namely_type;
use crate::parser::expr::r#type::Expr;

fn gen_env<'t>() -> (TypeEnv, ExprEnv<'t>) {
    let seq = "
        type A = Int
    ";
    parse_env(seq)
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr_type = Expr::Discard(namely_type!("A").some())
        .infer_type(&type_env, &expr_env);

    assert_eq!(expr_type, has_type(namely_type!("A")))
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr_type =
        Expr::Discard(None).infer_type(&type_env, &expr_env);

    assert_eq!(
        expr_type,
        RequireInfo::of("_", EnvRefConstraint::empty()).into()
    )
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr_type = Expr::Discard(namely_type!("B").some())
        .infer_type(&type_env, &expr_env);

    assert_matches!(expr_type, Quad::R(TypeMissMatch { .. }))
}
