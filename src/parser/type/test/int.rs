use crate::int_type;
use crate::parser::r#type::test::f;

#[test]
fn test_parse_int_type() {
    let r = int_type!();
    let r = Some(r);

    let seq = "Int";
    assert_eq!(f(seq), r);
    let seq = "(((Int)))";
    assert_eq!(f(seq), r);
}
