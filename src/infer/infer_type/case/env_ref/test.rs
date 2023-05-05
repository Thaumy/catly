use std::assert_matches::assert_matches;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::parse_to_env;
use crate::infer::env::r#macro::int_type;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::require_info::ReqInfo;
use crate::infer::infer_type::test::{
    check_has_type,
    check_req_constraint
};
use crate::infra::quad::Quad;

fn gen_env<'t>() -> (TypeEnv<'t>, ExprEnv<'t>) {
    let seq = "
        def a = _
        def b: Int = a
        def c = a: Int
        def d = 1
        def e: Int = _
        def f = _: Int
        def envRef7: Unit = d
        def envRef8: Unit = c

        def a9 = b9
        def b9 = _
        def envRef9: Unit = a9
    ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("a")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let r = ReqInfo::of("a", EnvRefConstraint::empty()).into();

    assert_eq!(infer_result, r)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("b")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    let erc = EnvRefConstraint::single("a".to_string(), int_type!());
    check_req_constraint!(infer_result, t, erc)
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("c")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    let erc = EnvRefConstraint::single("a".to_string(), int_type!());
    check_req_constraint!(infer_result, t, erc)
}

#[test]
fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("d")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    check_has_type!(infer_result, t)
}

#[test]
fn test_part5() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("e")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    check_has_type!(infer_result, t)
}

#[test]
fn test_part6() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("f")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    check_has_type!(infer_result, t)
}

#[test]
fn test_part7() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("envRef7")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R(..))
}

#[test]
fn test_part8() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("envRef8")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R(..))
}

#[test]
fn test_part9() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("envRef9")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::ML(..))
}
