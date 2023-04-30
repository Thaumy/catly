pub fn parse_unit(x: &str) -> bool { x == "()" }

#[test]
fn test_part1() {
    use crate::parser::value::unit::parse_unit;

    assert_eq!(parse_unit("()"), true);
    assert_eq!(parse_unit("abc"), false);
}
