use std::fmt;
use crate::parser::char::{parse_char, parse_lower};
use crate::parser::{get_head_tail};

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Pattern {
    Start,
    Lower(char),
    Char(char),
    LetName(String),
    End,
    Err,
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn go(prev_pat: Pattern, seq: &str, offset: i64) -> Option<(String, i64)> {
    let (head, tail) = get_head_tail(seq);
    let pat = match (&prev_pat, head) {
        // LetName: [0-9a-zA-Z] -> Char
        (Pattern::LetName(_), Some(c)) if parse_char(&c).is_some() =>
            Pattern::Char(c),
        // Start: [a-z] -> Lower
        (Pattern::Start, Some(c)) if parse_lower(&c).is_some() =>
            Pattern::Lower(c),
        // ɛ -> End
        (_, None) => Pattern::End,
        // _ -> Err
        (_, Some(c)) => {
            println!("Invalid head pattern: {}", c);
            Pattern::Err
        }
    };
    let this_pat = match (prev_pat, pat) {
        // Start Lower -> LetName
        (Pattern::Start, Pattern::Lower(c)) =>
            Pattern::LetName(c.to_string()),
        // LetName Char -> LetName
        (Pattern::LetName(n), Pattern::Char(c)) =>
            Pattern::LetName(format!("{}{}", n, c)),
        // Success
        (Pattern::LetName(n), Pattern::End) => return Some((n, offset)),
        // Can not parse
        (_, Pattern::Err) => return None,
        // Can not reduce
        (a, b) => {
            println!("Invalid reduce pattern: {}, {}", a, b);
            return None;
        }
    };
    go(this_pat, tail, offset + 1)
}

pub fn parse_let_name(seq: &str) -> Option<(String, i64)> {
    go(Pattern::Start, seq, 0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_let_name() {
        use crate::parser::name::let_name::parse_let_name;

        {
            let seq = "a1B2C3";
            let seq_len = seq.len() as i64;
            assert_eq!(
                parse_let_name(seq),
                Some((seq.to_string(), seq_len))
            );
        }
        assert_eq!(
            parse_let_name("A1b2c3"),
            None
        );
    }
}
