use std::assert_matches::assert_matches;

use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::get_type::r#fn::has_type;
use crate::get_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::get_type::r#type::require_constraint::require_constraint;
use crate::get_type::r#type::type_miss_match::TypeMissMatch;
use crate::get_type::test::parse_env;
use crate::infra::quad::Quad;
use crate::{closure_type, int_type, namely_type, unit_type};

fn gen_env<'t>() -> (TypeEnv, ExprEnv<'t>) {
    let seq = "
        def f1 = i -> i: Int
        def apply1 = f1 1

        def f2: Int -> Int -> Unit = a -> b -> ()
        def apply2 = f2 1 2

        def f3 = (a: Int) -> (b: Int) -> ()
        def apply3 = f3 1

        def apply4 = 1 1

        type F5 = Int -> Int
        def f5: F5 = a -> 1
        def apply5 = f5 1

        def b6 = _
        def apply6 = ((a: Int) -> 1) b6

        def apply7: Int = (_: Int -> Int) 1
        def apply8: Int = _ 1

        def apply9: Int = (a -> _) 1
        def apply10: Int = (a -> b -> c -> d -> 0) 1 2 3 4
        def apply11 = (a -> b -> c -> d -> 0) 1 2 3 4

        def a12 = a -> _
        def apply12: Int = a12 1
        def a13 = a -> b -> c -> d -> 0
        def apply13 = a13 1 2 3 4

        def apply14: Int -> Int = apply14
    ";
    parse_env(seq)
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply1")
        .unwrap();

    let r = has_type(int_type!());

    assert_eq!(expr.infer_type(&type_env, &expr_env), r)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply2")
        .unwrap();

    let r = has_type(unit_type!());

    assert_eq!(expr.infer_type(&type_env, &expr_env), r)
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply3")
        .unwrap();

    let r = has_type(closure_type!(int_type!(), unit_type!()));

    assert_eq!(expr.infer_type(&type_env, &expr_env), r)
}

#[test]
fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply4")
        .unwrap();

    assert_matches!(
        expr.infer_type(&type_env, &expr_env),
        Quad::R(TypeMissMatch { .. })
    )
}

#[test]
fn test_part5() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply5")
        .unwrap();

    assert_matches!(
        expr.infer_type(&type_env, &expr_env),
        Quad::R(TypeMissMatch { .. })
    )
}

#[test]
fn test_part6() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply6")
        .unwrap();

    let r = require_constraint(
        namely_type!("Int"),
        EnvRefConstraint::single(
            "b6".to_string(),
            namely_type!("Int")
        )
    );

    assert_eq!(expr.infer_type(&type_env, &expr_env), r)
}

#[test]
fn test_part7() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply7")
        .unwrap();

    let r = has_type(namely_type!("Int"));

    assert_eq!(expr.infer_type(&type_env, &expr_env), r)
}

#[test]
fn test_part8() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply8")
        .unwrap();

    let r = has_type(namely_type!("Int"));

    assert_eq!(expr.infer_type(&type_env, &expr_env), r)
}

#[test]
fn test_part9() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply9")
        .unwrap();

    let r = has_type(namely_type!("Int"));

    assert_eq!(expr.infer_type(&type_env, &expr_env), r)
}

#[test]
fn test_part10() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply10")
        .unwrap();

    let r = has_type(namely_type!("Int"));

    assert_eq!(expr.infer_type(&type_env, &expr_env), r)
}

#[test]
fn test_part11() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply11")
        .unwrap();

    let r = has_type(namely_type!("Int"));

    assert_eq!(expr.infer_type(&type_env, &expr_env), r)
}

#[test]
fn test_part12() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply12")
        .unwrap();

    let r = require_constraint(
        namely_type!("Int"),
        EnvRefConstraint::single(
            "a12".to_string(),
            closure_type!(int_type!(), int_type!())
        )
    );

    assert_eq!(expr.infer_type(&type_env, &expr_env), r)
}

#[test]
fn test_part13() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply13")
        .unwrap();

    let r = require_constraint(
        namely_type!("Int"),
        EnvRefConstraint::single(
            "a13".to_string(),
            closure_type!(
                int_type!(),
                closure_type!(
                    int_type!(),
                    closure_type!(
                        int_type!(),
                        closure_type!(int_type!(), int_type!())
                    )
                )
            )
        )
    );

    assert_eq!(expr.infer_type(&type_env, &expr_env), r)
}

#[test]
fn test_part14() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply14")
        .unwrap();

    let r = has_type(closure_type!(int_type!(), int_type!()));

    assert_eq!(expr.infer_type(&type_env, &expr_env), r)
}
