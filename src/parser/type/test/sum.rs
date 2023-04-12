use crate::parser::r#type::test::f;
use crate::parser::r#type::Type;
use crate::{btree_set, int_type, unit_type};

#[test]
fn test_parse_sum_type() {
    let r = Type::SumType(btree_set![
        Type::TypeEnvRef("A".to_string()),
        unit_type!(),
        Type::TypeEnvRef("C".to_string()),
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
