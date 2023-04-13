use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::parser::r#type::Type;
use crate::type_checker::get_type::get_type;
use crate::type_checker::get_type::r#type::EnvRefConstraint;
use crate::type_checker::get_type::test::parse_env;
use crate::{
    has_type,
    int_type,
    namely_type,
    require_constraint,
    type_miss_match,
    unit_type
};

fn gen_env<'t>() -> (TypeEnv, ExprEnv<'t>) {
    let seq = "
        def struct1 = { a = 1, b = () }
        def struct2: { a: Int, b: Unit } = { a = 1, b = () }

        type Prod = { a: Int, b: Unit }
        def struct3: Prod = { a = 1, b = () }

        def x = _
        def struct4: { a: Int, b: Unit } = { a = 1, b = x }
        def struct5 = { a = 1, b: Unit = x }

        def struct6 = { a = { a = 1 } }
        def struct7 = { a = { a = x: Int } }

        def struct8 = { a: Int = () }
        def struct9: Int = { a = 1 }
    ";
    parse_env(seq)
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("struct1")
        .unwrap();
    let r = has_type!(Type::ProdType(vec![
        ("a".to_string(), int_type!()),
        ("b".to_string(), unit_type!())
    ]));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("struct2")
        .unwrap();
    let r = has_type!(Type::ProdType(vec![
        ("a".to_string(), int_type!()),
        ("b".to_string(), unit_type!())
    ]));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("struct3")
        .unwrap();
    let r = has_type!(namely_type!("Prod"));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("struct4")
        .unwrap();
    let r = require_constraint!(
        Type::ProdType(vec![
            ("a".to_string(), int_type!()),
            ("b".to_string(), unit_type!())
        ]),
        EnvRefConstraint::single("x".to_string(), unit_type!())
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part5() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("struct5")
        .unwrap();
    let r = require_constraint!(
        Type::ProdType(vec![
            ("a".to_string(), int_type!()),
            ("b".to_string(), unit_type!())
        ]),
        EnvRefConstraint::single("x".to_string(), unit_type!())
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part6() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("struct6")
        .unwrap();
    let r = has_type!(Type::ProdType(vec![(
        "a".to_string(),
        Type::ProdType(vec![("a".to_string(), int_type!()),])
    )]));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part7() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("struct7")
        .unwrap();
    let r = require_constraint!(
        Type::ProdType(vec![(
            "a".to_string(),
            Type::ProdType(vec![("a".to_string(), int_type!()),])
        )]),
        EnvRefConstraint::single("x".to_string(), int_type!())
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part8() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("struct8")
        .unwrap();
    let r = type_miss_match!();

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part9() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("struct9")
        .unwrap();
    let r = type_miss_match!();

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}
