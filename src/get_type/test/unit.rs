use std::assert_matches::assert_matches;

use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::get_type::r#fn::has_type;
use crate::get_type::r#type::type_miss_match::TypeMissMatch;
use crate::get_type::test::parse_env;
use crate::infra::quad::Quad;
use crate::{namely_type, unit_type};

fn gen_env<'t>() -> (TypeEnv, ExprEnv<'t>) {
    let seq = "
        type A = Unit
        type B = Int
        def u = (): A
        def i = 10: A
        def k = ()
    ";
    parse_env(seq)
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_expr("u")
        .unwrap();

    assert_eq!(
        expr.infer_type(&type_env, &expr_env),
        has_type(namely_type!("A"))
    )
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_expr("i")
        .unwrap();

    assert_matches!(
        expr.infer_type(&type_env, &expr_env),
        Quad::R(TypeMissMatch { .. })
    )
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_expr("k")
        .unwrap();

    assert_eq!(
        expr.infer_type(&type_env, &expr_env),
        has_type(unit_type!())
    )
}
