use crate::parser::r#type::test::f;
use crate::parser::r#type::Type;
use crate::{btree_set, int_type, namely_type, unit_type};

#[test]
fn test_parse_sum_type() {
    let r = Type::SumType(btree_set![
        namely_type!("A"),
        unit_type!(),
        namely_type!("C"),
        int_type!(),
    ]);
    let r = Some(r);

    let seq = "A | Unit | C | Int";
    assert_eq!(f(seq), r);
    let seq = "(((A | Unit))) | (((C | Int)))";
    assert_eq!(f(seq), r);
    let seq = "A | (((Unit | C))) | Int";
    assert_eq!(f(seq), r);
    let seq = "A | (Unit | C | Int)";
    assert_eq!(f(seq), r);
    let seq = "A | (((Unit | C | Int)))";
    assert_eq!(f(seq), r);
}
