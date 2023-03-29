#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum Keyword {
    Type,
    Def,
    Let,
    In,
    If,
    Then,
    Else,
    Match,
    With,
}

impl Keyword {
    pub fn is_top_level(&self) -> bool {
        match self {
            Keyword::Type | Keyword::Def => true,
            _ => false,
        }
    }
}

pub fn parse_type(x: &str) -> bool {
    x == "type"
}

pub fn parse_def(x: &str) -> bool {
    x == "def"
}

pub fn parse_let(x: &str) -> bool {
    x == "let"
}

pub fn parse_in(x: &str) -> bool {
    x == "in"
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

pub fn parse_match(x: &str) -> bool {
    x == "match"
}

pub fn parse_with(x: &str) -> bool {
    x == "with"
}

pub fn parse_keyword(x: &str) -> Option<Keyword> {
    let map: [(fn(&str) -> bool, Keyword); 9] = [
        (parse_type, Keyword::Type),
        (parse_def, Keyword::Def),
        (parse_let, Keyword::Let),
        (parse_in, Keyword::In),
        (parse_if, Keyword::If),
        (parse_then, Keyword::Then),
        (parse_else, Keyword::Else),
        (parse_match, Keyword::Match),
        (parse_with, Keyword::With),
    ];
    map.iter()
        .find(|kv| kv.0(x))
        .and_then(|kv| Some(kv.1.clone()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_keyword() {
        use crate::parser::keyword::{Keyword, parse_keyword};

        assert_eq!(parse_keyword("type"), Some(Keyword::Type));
        assert_eq!(parse_keyword("def"), Some(Keyword::Def));
        assert_eq!(parse_keyword("let"), Some(Keyword::Let));
        assert_eq!(parse_keyword("in"), Some(Keyword::In));
        assert_eq!(parse_keyword("if"), Some(Keyword::If));
        assert_eq!(parse_keyword("then"), Some(Keyword::Then));
        assert_eq!(parse_keyword("else"), Some(Keyword::Else));
        assert_eq!(parse_keyword("match"), Some(Keyword::Match));
        assert_eq!(parse_keyword("with"), Some(Keyword::With));

        assert_eq!(parse_keyword("abc"), None);
    }
}
