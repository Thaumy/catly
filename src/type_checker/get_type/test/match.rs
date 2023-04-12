use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::type_checker::get_type::get_type;
use crate::type_checker::get_type::r#type::EnvRefConstraint;
use crate::type_checker::get_type::test::parse_env;
use crate::{has_type, int_type, require_constraint};

fn gen_env<'t>() -> (TypeEnv, ExprEnv<'t>) {
    let seq = "
        def match1 =
            match 1 with
            | 0 -> 1
            | 1 -> 2
            | _ -> 3

        def match2 =
            match 1 with
            | 0 -> 1
            | i -> i

        def a = 1
        def match3 =
            match a with
            | _ -> a

        def b = _
        def match4 =
           match b with
           | (i: Int) -> 1

        def c = _
        def match5 =
           match c with
           | _ -> (c: Int)
    ";
    parse_env(seq)
}

#[test]
pub fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("match1")
        .unwrap();

    let r = has_type!(int_type!());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("match2")
        .unwrap();

    let r = has_type!(int_type!());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("match3")
        .unwrap();

    let r = has_type!(int_type!());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("match4")
        .unwrap();
    let r = require_constraint!(
        int_type!(),
        EnvRefConstraint::single("b".to_string(), int_type!())
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part5() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("match5")
        .unwrap();
    let r = require_constraint!(
        int_type!(),
        EnvRefConstraint::single("c".to_string(), int_type!())
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}
