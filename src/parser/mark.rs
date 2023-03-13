#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Mark {
    Underline,
    Blank,
    Arrow,
    LeftParentheses,
    RightParentheses,
    LeftCurlyBracket,
    RightCurlyBracket,
    Colon,
    Asterisk,
    Comma,
    VerticalBar,
    Equal,
}

#[allow(dead_code)]
pub fn parse_underline(x: &char) -> bool {
    x == &'_'
}

#[allow(dead_code)]
pub fn parse_blank(x: &char) -> bool {
    x == &' '
}

#[allow(dead_code)]
pub fn parse_l_parentheses(x: &char) -> bool {
    x == &'('
}

#[allow(dead_code)]
pub fn parse_r_parentheses(x: &char) -> bool {
    x == &')'
}

#[allow(dead_code)]
pub fn parse_l_curly_bracket(x: &char) -> bool {
    x == &'{'
}

#[allow(dead_code)]
pub fn parse_r_curly_bracket(x: &char) -> bool {
    x == &'}'
}

#[allow(dead_code)]
pub fn parse_colon(x: &char) -> bool {
    x == &':'
}

#[allow(dead_code)]
pub fn parse_asterisk(x: &char) -> bool {
    x == &'*'
}

#[allow(dead_code)]
pub fn parse_comma(x: &char) -> bool {
    x == &','
}

#[allow(dead_code)]
pub fn parse_vertical_bar(x: &char) -> bool {
    x == &'|'
}

#[allow(dead_code)]
pub fn parse_equal(x: &char) -> bool {
    x == &'='
}

#[allow(dead_code)]
pub fn parse_arrow(x: &[char; 2]) -> bool {
    x == &['-', '>']
}

pub fn parse_mark(x: &str) -> Option<Mark> {
    let r = match x {
        "_" => Mark::Underline,
        " " => Mark::Blank,
        "(" => Mark::LeftParentheses,
        ")" => Mark::RightParentheses,
        "{" => Mark::LeftCurlyBracket,
        "}" => Mark::RightCurlyBracket,
        ":" => Mark::Colon,
        "*" => Mark::Asterisk,
        "," => Mark::Comma,
        "|" => Mark::VerticalBar,
        "=" => Mark::Equal,
        "->" => Mark::Arrow,
        _ => return None
    };
    Some(r)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_mark() {
        use crate::parser::mark::{Mark, parse_mark};

        assert_eq!(parse_mark("_"), Some(Mark::Underline));
        assert_eq!(parse_mark(" "), Some(Mark::Blank));
        assert_eq!(parse_mark("("), Some(Mark::LeftParentheses));
        assert_eq!(parse_mark(")"), Some(Mark::RightParentheses));
        assert_eq!(parse_mark("{"), Some(Mark::LeftCurlyBracket));
        assert_eq!(parse_mark("}"), Some(Mark::RightCurlyBracket));
        assert_eq!(parse_mark(":"), Some(Mark::Colon));
        assert_eq!(parse_mark(","), Some(Mark::Comma));
        assert_eq!(parse_mark("|"), Some(Mark::VerticalBar));
        assert_eq!(parse_mark("="), Some(Mark::Equal));
        assert_eq!(parse_mark("->"), Some(Mark::Arrow));

        assert_eq!(parse_mark("a"), None);
        assert_eq!(parse_mark("ab"), None);
    }
}