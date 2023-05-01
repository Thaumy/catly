use crate::infer::env::r#macro::unit_type;
use crate::infra::option::OptionAnyExt;
use crate::parser::expr::r#type::Expr;
use crate::parser::expr::test::f;

#[test]
fn test_part1() {
    let r = Expr::Unit(None);
    let r = Some(r);

    assert_eq!(f("()"), r);
    assert_eq!(f("(())"), r);
    assert_eq!(f("(((())))"), r);
}

#[test]
fn test_part2() {
    let r = Expr::Unit(unit_type!().some());
    let r = Some(r);

    assert_eq!(f("(): Unit"), r);
    assert_eq!(f("((): Unit)"), r);
    assert_eq!(f("((((): Unit)))"), r);
    assert_eq!(f("(): Unit"), r);
    assert_eq!(f("(()): Unit"), r);
    assert_eq!(f("(((()))): Unit"), r);
    assert_eq!(f("(): Unit"), r);
    assert_eq!(f("(()): (Unit)"), r);
    assert_eq!(f("(((()))): (((Unit)))"), r);
}
