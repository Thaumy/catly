use crate::parser::expr::Expr;
use crate::parser::expr::test::f;

#[test]
fn test_parse_env_ref() {
    let r = Expr::EnvRef("abc".to_string());
    let r = Some(r);

    assert_eq!(f("abc"), r);
    assert_eq!(f("(abc)"), r);
    assert_eq!(f("((abc))"), r);
}
