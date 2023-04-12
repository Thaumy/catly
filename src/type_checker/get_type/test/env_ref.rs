use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::infra::option::AnyExt;
use crate::parser::expr::Expr;
use crate::parser::r#type::Type;
use crate::type_checker::get_type::get_type;
use crate::type_checker::get_type::test::parse_env;
use crate::{has_type, require_constraint, require_info};

fn gen_env<'t>() -> (TypeEnv, ExprEnv<'t>) {
    let seq = "
        def a = _
        def b: Int = a
        def c = a: Int
        def d = 1
        def e: Int = _
        def f = _: Int
    ";
    parse_env(seq)
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env.get_ref("a").unwrap();

    let r = require_info!("a".to_string());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env.get_ref("b").unwrap();

    let r = require_constraint!(
        Type::TypeEnvRef("Int".to_string()),
        vec![("a".to_string(), Type::TypeEnvRef("Int".to_string()))]
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env.get_ref("c").unwrap();

    let r = require_constraint!(
        Type::TypeEnvRef("Int".to_string()),
        vec![("a".to_string(), Type::TypeEnvRef("Int".to_string()))]
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env.get_ref("d").unwrap();

    let r = has_type!(Type::TypeEnvRef("Int".to_string()));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part5() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env.get_ref("e").unwrap();

    let r = has_type!(Type::TypeEnvRef("Int".to_string()));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part6() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env.get_ref("f").unwrap();

    let r = has_type!(Type::TypeEnvRef("Int".to_string()));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}
