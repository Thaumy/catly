use crate::parser::define::test::f;
use crate::parser::define::Define;
use crate::parser::r#type::Type;
use crate::{namely_type, unit_type};

#[test]
fn test_parse_type_def_part1() {
    let r = Define::TypeDef("A".to_string(), namely_type!("B"));
    let r = Some(r);

    let seq = "type A = B";
    assert_eq!(f(seq), r)
}

#[test]
fn test_parse_type_def_part2() {
    let t = Type::ProdType(vec![
        ("abc".to_string(), namely_type!("A")),
        ("uuu".to_string(), unit_type!()),
        ("intList".to_string(), namely_type!("IntList")),
    ]);

    let r = Define::TypeDef("A".to_string(), t);
    let r = Some(r);

    let seq = "type A = { abc: A, uuu: Unit, intList: IntList }";
    assert_eq!(f(seq), r)
}
