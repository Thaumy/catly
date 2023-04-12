use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::parser::r#type::Type;
use crate::type_checker::get_type::get_type;
use crate::type_checker::get_type::test::parse_env;
use crate::{has_type, int_type, type_miss_match};

fn gen_env<'t>() -> (TypeEnv, ExprEnv<'t>) {
    let seq = "
        type A = Int
        type B = Unit
        def i = 10: A
        def u = (): A
        def k = 20
    ";
    parse_env(seq)
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_expr("i")
        .unwrap();

    assert_eq!(
        get_type(&type_env, &expr_env, &expr),
        has_type!(Type::TypeEnvRef("A".to_string()))
    )
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_expr("u")
        .unwrap();

    assert_eq!(
        get_type(&type_env, &expr_env, &expr),
        type_miss_match!()
    )
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_expr("k")
        .unwrap();

    assert_eq!(
        get_type(&type_env, &expr_env, &expr),
        has_type!(int_type!())
    )
}
