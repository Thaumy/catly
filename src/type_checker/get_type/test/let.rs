use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::type_checker::get_type::get_type;
use crate::type_checker::get_type::r#type::EnvRefConstraint;
use crate::type_checker::get_type::test::parse_env;
use crate::{
    has_type,
    int_type,
    require_constraint,
    require_info,
    unit_type
};

fn gen_env<'t>() -> (TypeEnv, ExprEnv<'t>) {
    let seq = "
        def let1 = let a = 1 in a

        def x = _
        def let2 = let a = 1 in x: Int
        def let3 = let a = x in a: Int

        def let4 = let a = 1, b = () in b

        def let5 = let a = _ in 1
        def let6 = let a = x in 1
    ";
    parse_env(seq)
}

#[test]
pub fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("let1")
        .unwrap();

    let r = has_type!(int_type!());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("let2")
        .unwrap();
    let r = require_constraint!(
        int_type!(),
        EnvRefConstraint::single("x".to_string(), int_type!())
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("let3")
        .unwrap();
    let r = require_constraint!(
        int_type!(),
        EnvRefConstraint::single("x".to_string(), int_type!())
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("let4")
        .unwrap();

    let r = has_type!(unit_type!());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part5() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("let5")
        .unwrap();

    let r = require_info!("a".to_string());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part6() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("let6")
        .unwrap();

    let r = require_info!("x".to_string());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}
