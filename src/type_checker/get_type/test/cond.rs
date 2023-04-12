use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::infra::option::AnyExt;
use crate::parser::expr::Expr;
use crate::parser::r#type::Type;
use crate::type_checker::get_type::get_type;
use crate::type_checker::get_type::test::parse_env;
use crate::{has_type, require_constraint, type_miss_match};

fn gen_env<'t>() -> (TypeEnv, ExprEnv<'t>) {
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

        def b = _
        def cond5 = (if b then _ else _): Unit

        def x = 1
        def y = _
        def cond6 = if false then x else y
    ";
    parse_env(seq)
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("cond1")
        .unwrap();

    let r = has_type!(Type::TypeEnvRef("Int".to_string()));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("cond2")
        .unwrap();

    let r = has_type!(Type::TypeEnvRef("Unit".to_string()));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("cond3")
        .unwrap();

    let r = type_miss_match!();

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("cond4")
        .unwrap();

    let r = type_miss_match!();

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part5() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("cond5")
        .unwrap();

    let r = require_constraint!(
        Type::TypeEnvRef("Unit".to_string()),
        vec![("b".to_string(), Type::TypeEnvRef("Bool".to_string()))]
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part6() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("cond6")
        .unwrap();

    let r = require_constraint!(
        Type::TypeEnvRef("Int".to_string()),
        vec![("y".to_string(), Type::TypeEnvRef("Int".to_string()))]
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}
