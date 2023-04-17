use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::get_type::get_type;
use crate::get_type::test::parse_env;
use crate::{
    has_type,
    int_type,
    require_constraint,
    require_info,
    single_constraint,
    type_miss_match
};

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

    let r = require_info!("a".to_string());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env.get_ref("b").unwrap();
    let r = require_constraint!(
        int_type!(),
        single_constraint!("a".to_string(), int_type!())
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env.get_ref("c").unwrap();
    let r = require_constraint!(
        int_type!(),
        single_constraint!("a".to_string(), int_type!())
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env.get_ref("d").unwrap();

    let r = has_type!(int_type!());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part5() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env.get_ref("e").unwrap();

    let r = has_type!(int_type!());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part6() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env.get_ref("f").unwrap();

    let r = has_type!(int_type!());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part7() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("a7")
        .unwrap();

    let r = type_miss_match!();

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part8() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("a8")
        .unwrap();

    let r = type_miss_match!();

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}
