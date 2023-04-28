use crate::infer::env::r#macro::unit_type;
use crate::parser::r#type::test::f;

#[test]
fn test_parse_unit_type() {
    let r = unit_type!();
    let r = Some(r);

    let seq = "Unit";
    assert_eq!(f(seq), r);
    let seq = "(((Unit)))";
    assert_eq!(f(seq), r);
}
