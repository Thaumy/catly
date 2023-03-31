use crate::btree_set;
use crate::parser::r#type::test::f;
use crate::parser::r#type::Type;

#[test]
fn test_parse_sum_type() {
    let r = Type::SumType(btree_set![
        Type::TypeEnvRef("A".to_string()),
        Type::TypeEnvRef("Unit".to_string()),
        Type::TypeEnvRef("C".to_string()),
        Type::TypeEnvRef("Int".to_string()),
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
