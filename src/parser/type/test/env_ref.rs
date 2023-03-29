use crate::parser::r#type::test::f;
use crate::parser::r#type::Type;

#[test]
fn test_f_env_ref() {
    let r = Type::TypeEnvRef("A".to_string());
    let r = Some(r);

    let seq = "A";
    assert_eq!(f(seq), r);
    let seq = "(((A)))";
    assert_eq!(f(seq), r);

    assert_eq!(f("a"), None);
    assert_eq!(f("1"), None);
}

#[test]
fn test_f_env_ref_part2() {
    let r = Type::TypeEnvRef("Abc123".to_string());
    let r = Some(r);

    let seq = "Abc123";
    assert_eq!(f(seq), r);
    let seq = "(((Abc123)))";
    assert_eq!(f(seq), r);

    assert_eq!(f("abc"), None);
    assert_eq!(f("123abc"), None);
}
