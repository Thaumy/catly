use crate::parser::r#type::test::f;
use crate::parser::r#type::Type;

#[test]
fn test_parse_int_type() {
    let r = Type::TypeEnvRef("Int".to_string());
    let r = Some(r);

    let seq = "Int";
    assert_eq!(f(seq), r);
    let seq = "(((Int)))";
    assert_eq!(f(seq), r);
}
