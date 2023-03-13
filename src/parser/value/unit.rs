pub fn parse_unit(x: &str) -> bool {
    x == "()"
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_unit() {
        use crate::parser::value::unit::parse_unit;

        assert_eq!(parse_unit("()"), true);
        assert_eq!(parse_unit("abc"), false);
    }
}
