use std::fmt;
use crate::parser::char::{parse_char, parse_upper};
use crate::parser::{get_head_tail};

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Pattern {
    Start,
    Upper(char),
    Char(char),
    TypeName(String),
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
        // TypeName: [0-9a-zA-Z] -> Char
        (Pattern::TypeName(_), Some(c)) if parse_char(&c).is_some() =>
            Pattern::Char(c),
        // Start: [A-Z] -> Upper
        (Pattern::Start, Some(c)) if parse_upper(&c).is_some() =>
            Pattern::Upper(c),
        // ɛ -> End
        (_, None) => Pattern::End,
        // _ -> Err
        (_, Some(c)) => {
            println!("Invalid head pattern: {}", c);
            Pattern::Err
        }
    };
    let this_pat = match (prev_pat, pat) {
        // Start Upper -> TypeName
        (Pattern::Start, Pattern::Upper(c)) =>
            Pattern::TypeName(c.to_string()),
        // TypeName Char -> TypeName
        (Pattern::TypeName(n), Pattern::Char(c)) =>
            Pattern::TypeName(format!("{}{}", n, c)),
        // Success
        (Pattern::TypeName(n), Pattern::End) => return Some((n, offset)),
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

pub fn parse_type_name(seq: &str) -> Option<(String, i64)> {
    go(Pattern::Start, seq, 0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_type_name() {
        use crate::parser::name::type_name::parse_type_name;

        assert_eq!(
            parse_type_name("a1B2C3"),
            None
        );
        {
            let seq = "A1b2c3";
            let seq_len = seq.len() as i64;
            assert_eq!(
                parse_type_name(seq),
                Some((seq.to_string(), seq_len))
            );
        }
    }
}
