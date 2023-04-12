use crate::namely_type;
use crate::parser::r#type::test::f;

#[test]
fn test_f_env_ref() {
    let r = namely_type!("A");
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
    let r = namely_type!("Abc123");
    let r = Some(r);

    let seq = "Abc123";
    assert_eq!(f(seq), r);
    let seq = "(((Abc123)))";
    assert_eq!(f(seq), r);

    assert_eq!(f("abc"), None);
    assert_eq!(f("123abc"), None);
}
