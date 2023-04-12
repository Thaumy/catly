use crate::infra::option::AnyExt;
use crate::parser::expr::test::f;
use crate::parser::expr::Expr;
use crate::parser::r#type::Type;
use crate::{btree_set, int_type, unit_type};

#[test]
fn test_parse_env_ref_part1() {
    let r = Expr::EnvRef(None, "abc".to_string());
    let r = Some(r);

    assert_eq!(f("abc"), r);
    assert_eq!(f("(abc)"), r);
    assert_eq!(f("((abc))"), r);
}

#[test]
fn test_parse_env_ref_part2() {
    let r = Expr::EnvRef(int_type!().some(), "abc".to_string());
    let r = Some(r);

    assert_eq!(f("abc: Int"), r);
    assert_eq!(f("(abc: Int)"), r);
    assert_eq!(f("(((abc: Int)))"), r);
    assert_eq!(f("abc: Int"), r);
    assert_eq!(f("(abc: (Int))"), r);
    assert_eq!(f("(((abc: (((Int))))))"), r);
}

#[test]
fn test_parse_env_ref_part3() {
    let r = Expr::EnvRef(
        Type::SumType(btree_set![
            Type::TypeEnvRef("A".to_string()),
            unit_type!(),
            int_type!(),
        ])
        .some(),
        "a".to_string()
    );
    let r = Some(r);

    assert_eq!(f("a: (A | Unit) | Int"), r);
    assert_eq!(f("(a: A | (Unit | Int))"), r);
    assert_eq!(f("(((a: ((A) | (Unit)) | (Int))))"), r);
    assert_eq!(f("a: (A) | ((Unit) | (Int))"), r);
    assert_eq!(f("(a: (((A | Unit)) | Int))"), r);
    assert_eq!(f("(((a: (((A | ((Unit | Int))))))))"), r);
}

#[test]
fn test_parse_env_ref_part4() {
    let r = Expr::EnvRef(
        Type::SumType(btree_set![
            Type::TypeEnvRef("A".to_string()),
            Type::TypeEnvRef("B".to_string()),
            Type::TypeEnvRef("C".to_string()),
            Type::TypeEnvRef("D".to_string()),
        ])
        .some(),
        "a".to_string()
    );
    let r = Some(r);

    assert_eq!(f("a: (A | B) | (C | D)"), r);
    assert_eq!(f("a: ((((A | B) | (C | D))))"), r);
    assert_eq!(f("(((a: (((A | B) | (C | D))))))"), r);
}
