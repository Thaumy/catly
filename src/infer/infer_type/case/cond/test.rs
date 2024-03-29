use std::assert_matches::assert_matches;
use std::rc::Rc;

use crate::infer::env::bool_type;
use crate::infer::env::int_type;
use crate::infer::env::parse_to_env;
use crate::infer::env::unit_type;
use crate::infer::env::ExprEnv;
use crate::infer::env::TypeEnv;
use crate::infer::infer_type::test::{
    check_has_type,
    check_req_constraint
};
use crate::infer::infer_type::EnvRefConstraint;
use crate::infra::Quad;

fn gen_env<'t>() -> (TypeEnv<'t>, Rc<ExprEnv>) {
    let seq = "
        type False = Int
        def false: False = 0

        type True = Int
        def true: True = 1

        type Bool = True | False

        def cond1 = if true then 1 else 0
        def cond2: Unit = if false then () else ()
        def cond3 = if () then 1 else 0
        def cond4 = if true then 1 else ()

        def b5 = _
        def cond5 = (if b5 then _ else _): Unit

        def a6 = 1
        def b6 = _
        def cond6 = if false then a6 else b6

        def cond7 = if false then _ else 1
        def a8 = _
        def cond8 = if false then _ else (a8: Int)
        def a9 = _
        def cond9 = if false then (a9: Int) else _

        def cond10 = if 0: Int then 1 else 1
        def a11 = _
        def cond11 = if a11: Int then 1 else 1
        def a12 = _
        def cond12 = if false then 1 else let a = a12 in _
        def cond13: Int = if false then (): Unit else 1
        def cond14: Int = if false then () else 1

        def a15 = _
        def cond15: Int = if let a = a15 in true then 1 else a15: Int
        def a16 = _
        def cond16: Int = if let a = a16 in true then 1 else 0
        def a17 = _
        def b17 = _
        def cond17: Int = if let a = a17 in true then let b: Int = b17 in 1 else 0
    ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("cond1")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    check_has_type!(infer_result, t)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("cond2")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = unit_type!();
    check_has_type!(infer_result, t)
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("cond3")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R(..))
}

#[test]
fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("cond4")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R(..))
}

#[test]
fn test_part5() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("cond5")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = unit_type!();
    let erc =
        EnvRefConstraint::single("b5".to_string(), bool_type!());
    check_req_constraint!(infer_result, t, erc)
}

#[test]
fn test_part6() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("cond6")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    let erc = EnvRefConstraint::single("b6".to_string(), int_type!());
    check_req_constraint!(infer_result, t, erc)
}

#[test]
fn test_part7() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("cond7")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    check_has_type!(infer_result, t)
}

#[test]
fn test_part8() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("cond8")
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
        .get_ref("cond9")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    let erc = EnvRefConstraint::single("a9".to_string(), int_type!());
    check_req_constraint!(infer_result, t, erc)
}

#[test]
fn test_part10() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("cond10")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R(..))
}

#[test]
fn test_part11() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("cond11")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R(..))
}

#[test]
fn test_part12() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("cond12")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::MR(..))
}

#[test]
fn test_part13() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("cond13")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R(..))
}

#[test]
fn test_part14() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("cond14")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R(..))
}

#[test]
fn test_part15() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("cond15")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    let erc =
        EnvRefConstraint::single("a15".to_string(), int_type!());
    check_req_constraint!(infer_result, t, erc)
}

#[test]
fn test_part16() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("cond16")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::MR(_))
}

#[test]
fn test_part17() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("cond17")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::MR(_))
}
