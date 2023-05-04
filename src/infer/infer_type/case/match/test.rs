use std::assert_matches::assert_matches;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::parse_env;
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

        def match11 =
            match _ with
            | 01 -> ()
            | () -> 01

        def match12 =
            match { a = 1, b = 2 } with
            | { a = x, b = x } -> 0

        def a13 = _
        def match13 =
            match 1 with
            | a13 1 -> _

        def a14 = _
        def match14: Int =
            match 1 with
            | 1 -> { a = a14 }

        def a15 = _
        def b15 = _
        def match15: Int =
            match 1 with
            | x -> let a = a15, b: Int = b15, c: Int = x in 1

        def match16 =
            match (): Int with
            | 1 -> 1
    ";
    parse_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match1")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    check_has_type!(expr_type, t)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match2")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    check_has_type!(expr_type, t)
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match3")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    check_has_type!(expr_type, t)
}

#[test]
fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match4")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    let erc = EnvRefConstraint::single("b".to_string(), int_type!());
    check_req_constraint!(expr_type, t, erc)
}

#[test]
fn test_part5() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match5")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    let erc = EnvRefConstraint::single("c".to_string(), int_type!());
    check_req_constraint!(expr_type, t, erc)
}

#[test]
fn test_part6() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match6")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    check_has_type!(expr_type, t)
}

#[test]
fn test_part7() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match7")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    check_has_type!(expr_type, t)
}

#[test]
fn test_part8() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match8")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = int_type!();
    let erc = EnvRefConstraint::single("a8".to_string(), int_type!());
    check_req_constraint!(expr_type, t, erc)
}

#[test]
fn test_part9() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match9")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r = ReqInfo::of("match9", EnvRefConstraint::empty()).into();

    assert_eq!(expr_type, r)
}

#[test]
fn test_part10() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match10")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(expr_type, Quad::R(..))
}

#[test]
fn test_part11() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match11")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r = ReqInfo::of("match11", EnvRefConstraint::empty()).into();

    assert_eq!(expr_type, r)
}

#[test]
fn test_part12() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match12")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(expr_type, Quad::R(..))
}

#[test]
fn test_part13() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match13")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(expr_type, Quad::R(..))
}

#[test]
fn test_part14() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match14")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(expr_type, Quad::R(..))
}

#[test]
fn test_part15() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match15")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(expr_type, Quad::MR(..))
}

#[test]
fn test_part16() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match16")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(expr_type, Quad::R(..))
}
