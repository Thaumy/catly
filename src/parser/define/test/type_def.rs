use crate::parser::define::test::f;
use crate::parser::define::Define;
use crate::{namely_type, prod_type, unit_type};

#[test]
fn test_part1() {
    let r = Define::TypeDef("A".to_string(), namely_type!("B"));
    let r = Some(r);

    let seq = "type A = B";
    assert_eq!(f(seq), r)
}

#[test]
fn test_part2() {
    let t = prod_type![
        ("abc".to_string(), namely_type!("A")),
        ("uuu".to_string(), unit_type!()),
        ("intList".to_string(), namely_type!("IntList")),
    ];

    let r = Define::TypeDef("A".to_string(), t);
    let r = Some(r);

    let seq = "type A = { abc: A, uuu: Unit, intList: IntList }";
    assert_eq!(f(seq), r)
}
