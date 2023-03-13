use std::fmt;
use crate::parser::{get_head_tail};

#[derive(Copy)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Pattern {
    Start,
    Int(i64),
    Digit(u8),
    End,
    Err,
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

//TODO: handle int overflow
fn go(prev_pat: Pattern, seq: &str, offset: i64) -> Option<(i64, i64)> {
    let (head, tail) = get_head_tail(seq);

    let pat = match head {
        // _ -> Digit/Err
        Some(c)
        => match crate::parser::char::parse_digit(&c) {
            // [0-9] -> Digit
            Some(d) => Pattern::Digit(d),
            // ɛ -> Err
            None => {
                println!("Invalid head pattern: {}", c);
                Pattern::Err
            }
        },
        // ɛ -> End
        None => Pattern::End
    };

    let this_pat = match (prev_pat, pat) {
        // Start Digit -> Int
        (Pattern::Start, Pattern::Digit(a)) => Pattern::Int(a as i64),
        // Int Digit -> Int
        (Pattern::Int(a), Pattern::Digit(b)) =>
            Pattern::Int(a * 10 + (b as i64)),
        // Success
        (Pattern::Int(a), Pattern::End) => return Some((a, offset)),
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

pub fn parse_int(x: &str) -> Option<(i64, i64)> {
    go(Pattern::Start, x, 0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_int() {
        use crate::parser::value::int::parse_int;

        assert_eq!(parse_int("abc"), None);
        assert_eq!(parse_int("1abc"), None);
        {
            let seq = "12345678";
            let seq_len = seq.len() as i64;
            assert_eq!(
                parse_int(seq),
                Some((12345678, seq_len))
            );
        }
    }
}
