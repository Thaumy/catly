use crate::parser::expr::Expr;
use crate::parser::expr::test::f;

#[test]
fn test_parse_unit() {
    let r = Expr::Unit(None);
    let r = Some(r);

    assert_eq!(f("()"), r);
    assert_eq!(f("(())"), r);
    assert_eq!(f("((()))"), r);
}
