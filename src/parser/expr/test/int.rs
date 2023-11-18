use crate::btree_set;
use crate::infer::env::r#macro::int_type;
use crate::infer::env::r#macro::namely_type;
use crate::infer::env::r#macro::sum_type;
use crate::infer::env::r#macro::unit_type;
use crate::infra::option::WrapOption;
use crate::parser::expr::r#type::Expr;
use crate::parser::expr::test::f;

#[test]
fn test_part1() {
    let r = Expr::Int(None, 123);
    let r = Some(r);

    assert_eq!(f("123"), r);
    assert_eq!(f("(123)"), r);
    assert_eq!(f("(((123)))"), r);
}

#[test]
fn test_part2() {
    let r = Expr::Int(int_type!().wrap_some(), 123);
    let r = Some(r);

    assert_eq!(f("123: Int"), r);
    assert_eq!(f("(123: Int)"), r);
    assert_eq!(f("(((123: Int)))"), r);
    assert_eq!(f("123: Int"), r);
    assert_eq!(f("(123: (Int))"), r);
    assert_eq!(f("(((123: (((Int))))))"), r);
}

#[test]
fn test_part3() {
    let r = Expr::Int(
        sum_type![int_type!(), unit_type!()].wrap_some(),
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
fn test_part4() {
    let r = Expr::Int(
        sum_type![int_type!(), unit_type!(), namely_type!("A")]
            .wrap_some(),
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
