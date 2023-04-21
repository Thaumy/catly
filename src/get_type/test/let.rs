use std::assert_matches::assert_matches;

use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::get_type::get_type;
use crate::get_type::r#fn::has_type;
use crate::get_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::get_type::r#type::require_constraint::require_constraint;
use crate::get_type::r#type::require_info::RequireInfo;
use crate::get_type::r#type::type_miss_match::TypeMissMatch;
use crate::get_type::test::parse_env;
use crate::infra::quad::Quad;
use crate::{int_type, unit_type};

fn gen_env<'t>() -> (TypeEnv, ExprEnv<'t>) {
    let seq = "
        def let1 = let a = 1 in a

        def x = _
        def let2 = let a = 1 in x: Int
        def let3 = let a = x in a: Int

        def let4 = let a = 1, b = () in b

        def let5 = let a = _ in 1
        def let6 = let a = x in 1

        def let7: Unit = let a = 1 in 1
        def a8 = _
        def let8 = let a: Int = a8 in 1

        def a9 = _
        def let9 = let a = _ in (a9: Int)
    ";
    parse_env(seq)
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("let1")
        .unwrap();
    let r = has_type(int_type!());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("let2")
        .unwrap();
    let r = require_constraint(
        int_type!(),
        EnvRefConstraint::single("x".to_string(), int_type!())
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("let3")
        .unwrap();
    let r = require_constraint(
        int_type!(),
        EnvRefConstraint::single("x".to_string(), int_type!())
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("let4")
        .unwrap();
    let r = has_type(unit_type!());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part5() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("let5")
        .unwrap();
    let r = RequireInfo::of("a").into();

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part6() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("let6")
        .unwrap();
    let r = RequireInfo::of("x").into();

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part7() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("let7")
        .unwrap();

    assert_matches!(
        get_type(&type_env, &expr_env, &expr),
        Quad::R(TypeMissMatch { .. })
    )
}

#[test]
fn test_part8() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("let8")
        .unwrap();
    let r = require_constraint(
        int_type!(),
        EnvRefConstraint::single("a8".to_string(), int_type!())
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part9() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("let9")
        .unwrap();
    let r = RequireInfo::of("a").into();

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}
