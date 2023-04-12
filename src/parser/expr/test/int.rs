use crate::infra::option::AnyExt;
use crate::parser::expr::test::f;
use crate::parser::expr::Expr;
use crate::parser::r#type::Type;
use crate::{btree_set, int_type, namely_type, unit_type};

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
    let r = Expr::Int(int_type!().some(), 123);
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
        Type::SumType(btree_set![int_type!(), unit_type!(),]).some(),
        123
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
            int_type!(),
            unit_type!(),
            namely_type!("A"),
        ])
        .some(),
        123
    );
    let r = Some(r);

    assert_eq!(f("123: Int | Unit | A"), r);
    assert_eq!(f("(123: Int | Unit | A)"), r);
    assert_eq!(f("(((123: Int | Unit | A)))"), r);
    assert_eq!(f("123: Int | Unit | A"), r);
    assert_eq!(f("(123: (Int | Unit | A))"), r);
    assert_eq!(f("(((123: (((Int | Unit | A))))))"), r);
}
