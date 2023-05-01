use crate::btree_set;
use crate::infer::env::r#macro::int_type;
use crate::infer::env::r#macro::namely_type;
use crate::infer::env::r#macro::sum_type;
use crate::infer::env::r#macro::unit_type;
use crate::infra::option::OptionAnyExt;
use crate::parser::expr::r#type::Expr;
use crate::parser::expr::test::f;

#[test]
fn test_part1() {
    let r = Expr::EnvRef(None, "abc".to_string());
    let r = Some(r);

    assert_eq!(f("abc"), r);
    assert_eq!(f("(abc)"), r);
    assert_eq!(f("((abc))"), r);
}

#[test]
fn test_part2() {
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
fn test_part3() {
    let r = Expr::EnvRef(
        sum_type![namely_type!("A"), unit_type!(), int_type!()]
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
fn test_part4() {
    let r = Expr::EnvRef(
        sum_type![
            namely_type!("A"),
            namely_type!("B"),
            namely_type!("C"),
            namely_type!("D"),
        ]
        .some(),
        "a".to_string()
    );
    let r = Some(r);

    assert_eq!(f("a: (A | B) | (C | D)"), r);
    assert_eq!(f("a: ((((A | B) | (C | D))))"), r);
    assert_eq!(f("(((a: (((A | B) | (C | D))))))"), r);
}
