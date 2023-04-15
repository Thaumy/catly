use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::type_checker::get_type::get_type;
use crate::type_checker::get_type::test::parse_env;
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

        def a6 = { a = 1, b = 2 }
        def match6 =
            match a6 with
            | { a = _, b = b } -> b
            | { a = a, b = _ } -> a

        def a7 = { a = 1, b = { x = 1, y = 2 } }
        def match7: Int =
            match a7 with
            | { a = a, b = _ } -> a
            | { a = _, b = { x = _, y = y } } -> y

        def a8 = _
        def match8: Int =
            match 1 with
            | 1 -> (a8: Int)
            | _ -> 0

        def match9 =
            match _ with
            | 01 -> ()
            | () -> 01
            | _ -> _

        def match10 =
            match 1 with
            | 01 -> ()
            | () -> 01
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
        single_constraint!("b".to_string(), int_type!())
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
        single_constraint!("c".to_string(), int_type!())
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part6() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("match6")
        .unwrap();
    let r = has_type!(int_type!());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part7() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("match7")
        .unwrap();
    let r = has_type!(int_type!());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part8() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("match8")
        .unwrap();
    let r = require_constraint!(
        int_type!(),
        single_constraint!("a8".to_string(), int_type!())
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part9() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("match9")
        .unwrap();
    let r = require_info!("match9".to_string());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part10() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("match10")
        .unwrap();
    let r = type_miss_match!();

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}