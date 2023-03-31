use crate::btree_set;
use crate::parser::expr::Expr;
use crate::parser::expr::test::f;
use crate::parser::infra::option::AnyExt;
use crate::parser::r#type::Type;

#[test]
fn test_parse_int_part1() {
    let r = Expr::Int(None, 123);
    let r = Some(r);

    assert_eq!(f("123"), r);
    assert_eq!(f("(123)"), r);
    assert_eq!(f("(((123)))"), r);
}

#[test]
fn test_parse_int_part2() {
    let r = Expr::Int(
        Type::TypeEnvRef("Int".to_string()).some(),
        123,
    );
    let r = Some(r);

    assert_eq!(f("123: Int"), r);
    assert_eq!(f("(123: Int)"), r);
    assert_eq!(f("(((123: Int)))"), r);
    assert_eq!(f("123: Int"), r);
    assert_eq!(f("(123: (Int))"), r);
    assert_eq!(f("(((123: (((Int))))))"), r);
}

#[test]
fn test_parse_int_part3() {
    let r = Expr::Int(
        Type::SumType(btree_set![
            Type::TypeEnvRef("Int".to_string()),
            Type::TypeEnvRef("Unit".to_string()),
        ]).some(),
        123,
    );
    let r = Some(r);

    assert_eq!(f("123: Int | Unit"), r);
    assert_eq!(f("(123: Int | Unit)"), r);
    assert_eq!(f("(((123: Int | Unit)))"), r);
    assert_eq!(f("123: Int | Unit"), r);
    assert_eq!(f("(123: (Int | Unit))"), r);
    assert_eq!(f("(((123: (((Int | Unit))))))"), r);
}

#[test]
fn test_parse_int_part4() {
    let r = Expr::Int(
        Type::SumType(btree_set![
            Type::TypeEnvRef("Int".to_string()),
            Type::TypeEnvRef("Unit".to_string()),
            Type::TypeEnvRef("A".to_string()),
        ]).some(),
        123,
    );
    let r = Some(r);

    assert_eq!(f("123: Int | Unit | A"), r);
    assert_eq!(f("(123: Int | Unit | A)"), r);
    assert_eq!(f("(((123: Int | Unit | A)))"), r);
    assert_eq!(f("123: Int | Unit | A"), r);
    assert_eq!(f("(123: (Int | Unit | A))"), r);
    assert_eq!(f("(((123: (((Int | Unit | A))))))"), r);
}
