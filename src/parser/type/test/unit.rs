use crate::parser::r#type::test::f;
use crate::unit_type;

#[test]
fn test_parse_unit_type() {
    let r = unit_type!();
    let r = Some(r);

    let seq = "Unit";
    assert_eq!(f(seq), r);
    let seq = "(((Unit)))";
    assert_eq!(f(seq), r);
}
