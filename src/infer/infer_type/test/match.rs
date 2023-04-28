use std::assert_matches::assert_matches;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::r#macro::int_type;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#fn::has_type;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::require_constraint::require_constraint;
use crate::infer::infer_type::r#type::require_info::RequireInfo;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infer::infer_type::test::parse_env;
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
    ";
    parse_env(seq)
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match1")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r = has_type(int_type!());

    assert_eq!(expr_type, r)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match2")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r = has_type(int_type!());

    assert_eq!(expr_type, r)
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match3")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r = has_type(int_type!());

    assert_eq!(expr_type, r)
}

#[test]
fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match4")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r = require_constraint(
        int_type!(),
        EnvRefConstraint::single("b".to_string(), int_type!())
    );

    assert_eq!(expr_type, r)
}

#[test]
fn test_part5() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match5")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r = require_constraint(
        int_type!(),
        EnvRefConstraint::single("c".to_string(), int_type!())
    );

    assert_eq!(expr_type, r)
}

#[test]
fn test_part6() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match6")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r = has_type(int_type!());

    assert_eq!(expr_type, r)
}

#[test]
fn test_part7() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match7")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r = has_type(int_type!());

    assert_eq!(expr_type, r)
}

#[test]
fn test_part8() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match8")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r = require_constraint(
        int_type!(),
        EnvRefConstraint::single("a8".to_string(), int_type!())
    );

    assert_eq!(expr_type, r)
}

#[test]
fn test_part9() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match9")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r =
        RequireInfo::of("match9", EnvRefConstraint::empty()).into();

    assert_eq!(expr_type, r)
}

#[test]
fn test_part10() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match10")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(expr_type, Quad::R(TypeMissMatch { .. }))
}

#[test]
fn test_part11() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("match11")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r =
        RequireInfo::of("match11", EnvRefConstraint::empty()).into();

    assert_eq!(expr_type, r)
}
