use std::assert_matches::assert_matches;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#fn::has_type;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::require_constraint::require_constraint;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infer::infer_type::test::parse_env;
use crate::infra::quad::Quad;
use crate::{bool_type, int_type, unit_type};

fn gen_env<'t>() -> (TypeEnv<'t>, ExprEnv<'t>) {
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

        def x = 1
        def y = _
        def cond6 = if false then x else y

        def cond7 = if false then _ else 1
        def a8 = _
        def cond8 = if false then _ else (a8: Int)
        def a9 = _
        def cond9 = if false then (a9: Int) else _
    ";
    parse_env(seq)
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("cond1")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r = has_type(int_type!());

    assert_eq!(expr_type, r)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("cond2")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r = has_type(unit_type!());

    assert_eq!(expr_type, r)
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("cond3")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(expr_type, Quad::R(TypeMissMatch { .. }))
}

#[test]
fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("cond4")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(expr_type, Quad::R(TypeMissMatch { .. }))
}

#[test]
fn test_part5() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("cond5")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r = require_constraint(
        unit_type!(),
        EnvRefConstraint::single("b5".to_string(), bool_type!())
    );

    assert_eq!(expr_type, r)
}

#[test]
fn test_part6() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("cond6")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r = require_constraint(
        int_type!(),
        EnvRefConstraint::single("y".to_string(), int_type!())
    );

    assert_eq!(expr_type, r)
}

#[test]
fn test_part7() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("cond7")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r = has_type(int_type!());

    assert_eq!(expr_type, r)
}

#[test]
fn test_part8() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("cond8")
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
        .get_ref("cond9")
        .unwrap()
        .infer_type(&type_env, &expr_env);
    let r = require_constraint(
        int_type!(),
        EnvRefConstraint::single("a9".to_string(), int_type!())
    );

    assert_eq!(expr_type, r)
}
