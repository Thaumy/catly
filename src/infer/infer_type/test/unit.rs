use std::assert_matches::assert_matches;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#fn::has_type;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infer::infer_type::test::parse_env;
use crate::infra::quad::Quad;
use crate::{namely_type, unit_type};

fn gen_env<'t>() -> (TypeEnv<'t>, ExprEnv<'t>) {
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

    let expr_type = expr_env
        .get_expr("u")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_eq!(expr_type, has_type(namely_type!("A")))
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_expr("i")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(expr_type, Quad::R(TypeMissMatch { .. }))
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_expr("k")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_eq!(expr_type, has_type(unit_type!()))
}
