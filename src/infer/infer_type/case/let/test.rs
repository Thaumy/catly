use std::assert_matches::assert_matches;
use std::rc::Rc;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::int_type;
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
        def let1 = let a = 1 in a

        def a2 = _
        def let2 = let a = 1 in a2: Int
        def a3 = _
        def let3 = let a = a3 in a: Int

        def let4 = let a = 1, b = () in b

        def let5 = let a = _ in 1
        def let6 = let a = x in 1

        def let7: Unit = let a = 1 in 1
        def a8 = _
        def let8 = let a: Int = a8 in 1

        def a9 = _
        def let9 = let a = _ in (a9: Int)

        def a10 = 1
        def let10: Unit = let a = _ in a10: Int

        def let11 = let a: Unit = 1 in 1
        def let12 = let a = let b = _ in _ in a: Int

        def a13 = _
        def let13: Unit = let a = _ in a13: Int

        def a14 = _
        def let14 = let a = let b = a14: Unit in _ in { x = a14: Int, y: Int = a }

        def let15 = let a = _ in a: Int
        def let16: Int = let a = 1 in (): Unit
        def let17 = let a: Unit = 1: Int in 1
        def let18 = let a = 1 in let a = a in a

        def add19: Int -> Int -> Int = _
        def a19 = _
        def let19 = let a19 = add19 a19 1 in 1

        def add20: Int -> Int -> Int = _
        def let20 = let rec a20 = add20 a20 1 in 1
    ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("let1")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    check_has_type!(infer_result, t)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("let2")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    let erc = EnvRefConstraint::single("a2".to_string(), int_type!());
    check_req_constraint!(infer_result, t, erc)
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("let3")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    let erc = EnvRefConstraint::single("a3".to_string(), int_type!());
    check_req_constraint!(infer_result, t, erc)
}

#[test]
fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("let4")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = unit_type!();
    check_has_type!(infer_result, t)
}

#[test]
fn test_part5() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("let5")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r = ReqInfo::of("a", EnvRefConstraint::empty()).into();

    assert_eq!(infer_result, r)
}

#[test]
fn test_part6() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("let6")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r = ReqInfo::of("x", EnvRefConstraint::empty()).into();

    assert_eq!(infer_result, r)
}

#[test]
fn test_part7() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("let7")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R(..))
}

#[test]
fn test_part8() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("let8")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    let erc = EnvRefConstraint::single("a8".to_string(), int_type!());
    check_req_constraint!(infer_result, t, erc)
}

#[test]
fn test_part9() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("let9")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r = ReqInfo::of(
        "a",
        EnvRefConstraint::single("a9".to_string(), int_type!())
    )
    .into();

    assert_eq!(infer_result, r)
}

#[test]
fn test_part10() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("let10")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R(..))
}

#[test]
fn test_part11() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("let11")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R(..))
}

#[test]
fn test_part12() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("let12")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::MR(..))
}

#[test]
fn test_part13() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("let13")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R(..))
}

#[test]
fn test_part14() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("let14")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R(..))
}

#[test]
fn test_part15() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("let15")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    check_has_type!(infer_result, t)
}

#[test]
fn test_part16() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("let16")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R(..))
}

#[test]
fn test_part17() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("let17")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R(..))
}

#[test]
fn test_part18() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("let18")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    check_has_type!(infer_result, t)
}

#[test]
fn test_part19() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("let19")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    let erc =
        EnvRefConstraint::single("a19".to_string(), int_type!());
    check_req_constraint!(infer_result, t, erc)
}

#[test]
fn test_part20() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("let20")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    check_has_type!(infer_result, t)
}
