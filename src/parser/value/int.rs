use std::{fmt, vec};
use crate::parser::{get_head_tail};

#[derive(Copy)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Pattern {
    Start,
    End,
    Err,

    Int(i64),
    Digit(u8),
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

//TODO: handle int overflow
fn go(stack: Vec<Pattern>, seq: &str) -> Option<i64> {
    let (head, tail) = get_head_tail(seq);

    let move_in = match head {
        // _ -> Digit/Err
        Some(c) =>
            match crate::parser::char::parse_digit(&c) {
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

    let reduced_stack = match (&stack[..], move_in) {
        // Start Digit -> Int
        ([Pattern::Start], Pattern::Digit(a)) => vec![Pattern::Int(a as i64)],
        // Int Digit -> Int
        ([Pattern::Int(a)], Pattern::Digit(b)) =>
            vec![Pattern::Int(a * 10 + (b as i64))],

        // Success
        ([Pattern::Int(a)], Pattern::End) => return Some(*a),

        // Can not parse
        (_, Pattern::Err) => return None,
        // Can not reduce
        (_, b) => {
            println!("Invalid reduce pattern: {:?}, {}", stack, b);
            return None;
        }
    };

    go(reduced_stack, tail)
}

pub fn parse_int(x: &str) -> Option<i64> {
    go(vec![Pattern::Start], x)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_int() {
        use crate::parser::value::int::parse_int;

        assert_eq!(parse_int("abc"), None);
        assert_eq!(parse_int("1abc"), None);
        assert_eq!(parse_int("12345678"), Some(12345678));
    }
}
