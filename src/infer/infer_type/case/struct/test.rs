use std::assert_matches::assert_matches;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::parse_to_env;
use crate::infer::env::r#macro::int_type;
use crate::infer::env::r#macro::namely_type;
use crate::infer::env::r#macro::prod_type;
use crate::infer::env::r#macro::unit_type;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::test::{
    check_has_type,
    check_req_constraint
};
use crate::infra::quad::Quad;

fn gen_env<'t>() -> (TypeEnv<'t>, ExprEnv<'t>) {
    let seq = "
        def struct1 = { a = 1, b = () }
        def struct2: { a: Int, b: Unit } = { a = 1, b = () }

        type Prod3 = { a: Int, b: Unit }
        def struct3: Prod3 = { a = 1, b = () }

        def x = _
        def struct4: { a: Int, b: Unit } = { a = 1, b = x }
        def struct5 = { a = 1, b: Unit = x }

        def struct6 = { a = { a = 1 } }
        def struct7 = { a = { a = x: Int } }

        def struct8 = { a: Int = () }
        def struct9: Int = { a = 1 }

        def a10 = _
        def struct10 = { a = a10, b = (a10: Int) }

        def struct11: T11 = { a = 1 }
        type Sum12 = { a: Int } | Int
        def struct12: Sum12 = { a = 1 }
        def a13 = _
        def struct13 = { a = a13: Int, b = let k = _ in a13: Unit }
    ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("struct1")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = prod_type![
        ("a".to_string(), int_type!()),
        ("b".to_string(), unit_type!())
    ];
    check_has_type!(infer_result, t)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("struct2")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = prod_type![
        ("a".to_string(), int_type!()),
        ("b".to_string(), unit_type!())
    ];
    check_has_type!(infer_result, t)
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("struct3")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = namely_type!("Prod3");
    check_has_type!(infer_result, t)
}

#[test]
fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("struct4")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = prod_type![
        ("a".to_string(), int_type!()),
        ("b".to_string(), unit_type!())
    ];
    let erc = EnvRefConstraint::single("x".to_string(), unit_type!());
    check_req_constraint!(infer_result, t, erc)
}

#[test]
fn test_part5() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("struct5")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = prod_type![
        ("a".to_string(), int_type!()),
        ("b".to_string(), unit_type!())
    ];
    let erc = EnvRefConstraint::single("x".to_string(), unit_type!());
    check_req_constraint!(infer_result, t, erc)
}

#[test]
fn test_part6() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("struct6")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = prod_type![("a".to_string(), prod_type![(
        "a".to_string(),
        int_type!()
    ),])];
    check_has_type!(infer_result, t)
}

#[test]
fn test_part7() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("struct7")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = prod_type![("a".to_string(), prod_type![(
        "a".to_string(),
        int_type!()
    ),])];
    let erc = EnvRefConstraint::single("x".to_string(), int_type!());
    check_req_constraint!(infer_result, t, erc)
}

#[test]
fn test_part8() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("struct8")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R(..))
}

#[test]
fn test_part9() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("struct9")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R(..))
}

#[test]
fn test_part10() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("struct10")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = prod_type![
        ("a".to_string(), int_type!()),
        ("b".to_string(), int_type!())
    ];
    let erc =
        EnvRefConstraint::single("a10".to_string(), int_type!());
    check_req_constraint!(infer_result, t, erc)
}

#[test]
fn test_part11() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("struct11")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R { .. })
}

#[test]
fn test_part12() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("struct12")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = namely_type!("Sum12");
    check_has_type!(infer_result, t)
}

#[test]
fn test_part13() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("struct13")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R { .. });
}
