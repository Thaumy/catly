use std::assert_matches::assert_matches;

use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::get_type::r#fn::has_type;
use crate::get_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::get_type::r#type::require_constraint::require_constraint;
use crate::get_type::r#type::require_info::RequireInfo;
use crate::get_type::r#type::type_miss_match::TypeMissMatch;
use crate::get_type::test::parse_env;
use crate::infra::quad::Quad;
use crate::int_type;

fn gen_env<'t>() -> (TypeEnv, ExprEnv<'t>) {
    let seq = "
        def a = _
        def b: Int = a
        def c = a: Int
        def d = 1
        def e: Int = _
        def f = _: Int
        def a7: Unit = d
        def a8: Unit = c
    ";
    parse_env(seq)
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env.get_ref("a").unwrap();

    let r = RequireInfo::of("a").into();

    assert_eq!(expr.infer_type(&type_env, &expr_env), r)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env.get_ref("b").unwrap();
    let r = require_constraint(
        int_type!(),
        EnvRefConstraint::single("a".to_string(), int_type!())
    );

    assert_eq!(expr.infer_type(&type_env, &expr_env), r)
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env.get_ref("c").unwrap();
    let r = require_constraint(
        int_type!(),
        EnvRefConstraint::single("a".to_string(), int_type!())
    );

    assert_eq!(expr.infer_type(&type_env, &expr_env), r)
}

#[test]
fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env.get_ref("d").unwrap();

    let r = has_type(int_type!());

    assert_eq!(expr.infer_type(&type_env, &expr_env), r)
}

#[test]
fn test_part5() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env.get_ref("e").unwrap();

    let r = has_type(int_type!());

    assert_eq!(expr.infer_type(&type_env, &expr_env), r)
}

#[test]
fn test_part6() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env.get_ref("f").unwrap();

    let r = has_type(int_type!());

    assert_eq!(expr.infer_type(&type_env, &expr_env), r)
}

#[test]
fn test_part7() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("a7")
        .unwrap();

    assert_matches!(
        expr.infer_type(&type_env, &expr_env),
        Quad::R(TypeMissMatch { .. })
    )
}

#[test]
fn test_part8() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("a8")
        .unwrap();

    assert_matches!(
        expr.infer_type(&type_env, &expr_env),
        Quad::R(TypeMissMatch { .. })
    )
}
