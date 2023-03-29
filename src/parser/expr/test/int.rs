use crate::parser::expr::Expr;
use crate::parser::expr::test::f;

#[test]
fn test_parse_int() {
    let r = Expr::Int(None, 123);
    let r = Some(r);

    assert_eq!(f("123"), r);
    assert_eq!(f("(123)"), r);
    assert_eq!(f("((123))"), r);
}

