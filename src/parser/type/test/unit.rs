use crate::parser::r#type::test::f;
use crate::parser::r#type::Type;

#[test]
fn test_parse_unit_type() {
    let r = Type::TypeEnvRef("Unit".to_string());
    let r = Some(r);

    let seq = "Unit";
    assert_eq!(f(seq), r);
    let seq = "(((Unit)))";
    assert_eq!(f(seq), r);
}
