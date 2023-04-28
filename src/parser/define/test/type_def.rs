use crate::infer::env::r#macro::namely_type;
use crate::infer::env::r#macro::prod_type;
use crate::infer::env::r#macro::unit_type;
use crate::parser::define::test::f;
use crate::parser::define::Define;

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
