use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::get_type::get_type;
use crate::get_type::test::parse_env;
use crate::{
    closure_type,
    has_type,
    int_type,
    namely_type,
    require_constraint,
    require_info,
    single_constraint,
    unit_type
};

fn gen_env<'t>() -> (TypeEnv, ExprEnv<'t>) {
    let seq = "
        def f1 = a -> a
        def f2: Int -> Int = a -> a
        def f3 = (a: Int) -> a
        def f4 = a -> a: Int

        def b = _
        def f5: Int -> Int = a -> b

        def f6: Int -> Int -> Unit = a -> a -> ()
        type F = Int -> Int -> Unit
        def f7: F = a -> a -> ()

        def f8: (Int -> Unit) -> Int = f -> 1

        def x = _
        def f9: Int -> Int -> Int = a -> x -> x
        def f10: Int -> (Int -> Int) = a -> (b -> x)

        def b11 = _
        def f11 = a -> (b11: Int)

        def f12 = _ -> 1
        def a13 = _
        def f13 = _ -> (a13: Int)

        def f14 = (_: Int) -> 0
        def f15: Int -> Int = _ -> 0
    ";
    parse_env(seq)
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f1")
        .unwrap();
    let r = require_info!("a".to_string());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f2")
        .unwrap();
    let r = has_type!(closure_type!(int_type!(), int_type!()));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f3")
        .unwrap();
    let r = has_type!(closure_type!(int_type!(), int_type!()));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f4")
        .unwrap();
    let r = has_type!(closure_type!(int_type!(), int_type!()));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part5() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f5")
        .unwrap();
    let r = require_constraint!(
        closure_type!(int_type!(), int_type!()),
        single_constraint!("b".to_string(), int_type!())
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part6() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f6")
        .unwrap();
    let r = has_type!(closure_type!(
        int_type!(),
        closure_type!(int_type!(), unit_type!())
    ));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part7() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f7")
        .unwrap();
    let r = has_type!(namely_type!("F"));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part8() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f8")
        .unwrap();
    let r = has_type!(closure_type!(
        closure_type!(int_type!(), unit_type!()),
        int_type!()
    ));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part9() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f9")
        .unwrap();
    let r = has_type!(closure_type!(
        int_type!(),
        closure_type!(int_type!(), int_type!())
    ));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part10() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f10")
        .unwrap();
    let r = require_constraint!(
        closure_type!(
            int_type!(),
            closure_type!(int_type!(), int_type!())
        ),
        single_constraint!("x".to_string(), int_type!())
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part11() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f11")
        .unwrap();
    let r = require_info!("a".to_string());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part12() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f12")
        .unwrap();
    let r = require_info!("_ (closure input)".to_string());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part13() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f13")
        .unwrap();
    let r = require_info!("_ (closure input)".to_string());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part14() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f14")
        .unwrap();
    let r = has_type!(closure_type!(int_type!(), int_type!()));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part15() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f15")
        .unwrap();
    let r = has_type!(closure_type!(int_type!(), int_type!()));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}
