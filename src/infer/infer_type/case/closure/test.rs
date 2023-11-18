use std::assert_matches::assert_matches;
use std::rc::Rc;

use crate::infer::env::closure_type;
use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::int_type;
use crate::infer::env::namely_type;
use crate::infer::env::parse_to_env;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::env::unit_type;
use crate::infer::infer_type::test::{
    check_has_type,
    check_req_constraint
};
use crate::infer::infer_type::EnvRefConstraint;
use crate::infer::infer_type::ReqInfo;
use crate::infra::quad::Quad;

fn gen_env<'t>() -> (TypeEnv<'t>, Rc<ExprEnv>) {
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
        def f16: Int -> Int = _ -> ()
        def f17: T17 = _ -> 1 # T17 should not be found in type env
        def f18: Unit = _ -> ()
    ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("f1")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r = ReqInfo::of("a", EnvRefConstraint::empty()).into();

    assert_eq!(infer_result, r)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("f2")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(int_type!(), int_type!());
    check_has_type!(infer_result, t)
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("f3")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(int_type!(), int_type!());
    check_has_type!(infer_result, t)
}

#[test]
fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("f4")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(int_type!(), int_type!());
    check_has_type!(infer_result, t)
}

#[test]
fn test_part5() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("f5")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(int_type!(), int_type!());
    let erc = EnvRefConstraint::single("b".to_string(), int_type!());
    check_req_constraint!(infer_result, t, erc)
}

#[test]
fn test_part6() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("f6")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(
        int_type!(),
        closure_type!(int_type!(), unit_type!())
    );
    check_has_type!(infer_result, t)
}

#[test]
fn test_part7() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("f7")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = namely_type!("F");
    check_has_type!(infer_result, t)
}

#[test]
fn test_part8() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("f8")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(
        closure_type!(int_type!(), unit_type!()),
        int_type!()
    );
    check_has_type!(infer_result, t)
}

#[test]
fn test_part9() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("f9")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(
        int_type!(),
        closure_type!(int_type!(), int_type!())
    );
    check_has_type!(infer_result, t)
}

#[test]
fn test_part10() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("f10")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(
        int_type!(),
        closure_type!(int_type!(), int_type!())
    );
    let erc = EnvRefConstraint::single("x".to_string(), int_type!());
    check_req_constraint!(infer_result, t, erc)
}

#[test]
fn test_part11() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("f11")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r = ReqInfo::of(
        "a",
        EnvRefConstraint::single("b11".to_string(), int_type!())
    )
    .into();

    assert_eq!(infer_result, r)
}

#[test]
fn test_part12() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("f12")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r =
        ReqInfo::of("_ (closure input)", EnvRefConstraint::empty())
            .into();

    assert_eq!(infer_result, r)
}

#[test]
fn test_part13() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("f13")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r = ReqInfo::of(
        "_ (closure input)",
        EnvRefConstraint::single("a13".to_string(), int_type!())
    )
    .into();

    assert_eq!(infer_result, r)
}

#[test]
fn test_part14() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("f14")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(int_type!(), int_type!());
    check_has_type!(infer_result, t)
}

#[test]
fn test_part15() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("f15")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(int_type!(), int_type!());
    check_has_type!(infer_result, t)
}

#[test]
fn test_part16() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("f16")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R { .. })
}

#[test]
fn test_part17() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("f17")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R { .. })
}

#[test]
fn test_part18() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("f18")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R { .. })
}
