use crate::btree_set;
use crate::parser::expr::Expr;
use crate::parser::expr::test::f;
use crate::parser::infra::option::AnyExt;
use crate::parser::r#type::Type;

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
    let r = Expr::EnvRef(
        Type::TypeEnvRef("Int".to_string()).some(),
        "abc".to_string(),
    );
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
            Type::TypeEnvRef("Unit".to_string()),
            Type::TypeEnvRef("Int".to_string()),
        ]).some(),
        "a".to_string(),
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
        ]).some(),
        "a".to_string(),
    );
    let r = Some(r);

    assert_eq!(f("a: (A | B) | (C | D)"), r);
    assert_eq!(f("a: ((((A | B) | (C | D))))"), r);
    assert_eq!(f("(((a: (((A | B) | (C | D))))))"), r);
}
