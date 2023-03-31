use crate::parser::expr::Expr;
use crate::parser::expr::test::f;
use crate::parser::infra::option::AnyExt;
use crate::parser::r#type::Type;

#[test]
fn test_parse_unit_part1() {
    let r = Expr::Unit(None);
    let r = Some(r);

    assert_eq!(f("()"), r);
    assert_eq!(f("(())"), r);
    assert_eq!(f("(((())))"), r);
}

#[test]
fn test_parse_unit_part2() {
    let r = Expr::Unit(
        Type::TypeEnvRef("Unit".to_string()).some()
    );
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
