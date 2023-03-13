#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum KeyWord {
    Let,
    Type,
    If,
    Then,
    Else,
    Case,
    Of,
}

pub fn parse_let(x: &str) -> bool {
    x == "let"
}

pub fn parse_type(x: &str) -> bool {
    x == "type"
}

pub fn parse_if(x: &str) -> bool {
    x == "if"
}

pub fn parse_then(x: &str) -> bool {
    x == "then"
}

pub fn parse_else(x: &str) -> bool {
    x == "else"
}

pub fn parse_case(x: &str) -> bool {
    x == "case"
}

pub fn parse_of(x: &str) -> bool {
    x == "of"
}

pub fn parse_keyword(x: &str) -> Option<KeyWord> {
    let map: [(fn(&str) -> bool, KeyWord); 7] = [
        (parse_let, KeyWord::Let),
        (parse_type, KeyWord::Type),
        (parse_if, KeyWord::If),
        (parse_then, KeyWord::Then),
        (parse_else, KeyWord::Else),
        (parse_case, KeyWord::Case),
        (parse_of, KeyWord::Of),
    ];
    map.iter()
        .find(|kv| kv.0(x))
        .and_then(|kv| Some(kv.1.clone()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_keyword() {
        use crate::parser::keyword::{KeyWord, parse_keyword};

        assert_eq!(parse_keyword("let"), Some(KeyWord::Let));
        assert_eq!(parse_keyword("type"), Some(KeyWord::Type));
        assert_eq!(parse_keyword("if"), Some(KeyWord::If));
        assert_eq!(parse_keyword("then"), Some(KeyWord::Then));
        assert_eq!(parse_keyword("else"), Some(KeyWord::Else));
        assert_eq!(parse_keyword("case"), Some(KeyWord::Case));
        assert_eq!(parse_keyword("of"), Some(KeyWord::Of));

        assert_eq!(parse_keyword("abc"), None);
    }
}
