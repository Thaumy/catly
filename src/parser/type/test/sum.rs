use crate::btree_set;
use crate::infer::env::int_type;
use crate::infer::env::namely_type;
use crate::infer::env::sum_type;
use crate::infer::env::unit_type;
use crate::parser::r#type::test::f;

#[test]
fn test_part1() {
    let r = sum_type![
        namely_type!("A"),
        unit_type!(),
        namely_type!("C"),
        int_type!(),
    ];
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
