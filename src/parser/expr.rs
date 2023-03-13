use std::fmt;
use crate::parser::char::{parse_char, parse_digit, parse_lower};
use crate::parser::get_head_tail_next;
use crate::parser::mark::{parse_l_parentheses, parse_r_parentheses};
use crate::parser::name::let_name::parse_let_name;
use crate::parser::value::int::parse_int;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Expr {
    Unit,
    Int(i64),
    EnvRef(String),
    //Apply,
    //Closure,
    //Cond,
    //Struct,
    //Match,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Pattern {
    Start,
    End,
    Err,

    Any(char),
    AnySeq(String),

    Digit(char),
    DigitSeq(String),
    Int(i64),

    Lower(char),
    Char(char),
    CharSeq(String),
    LetName(String),

    LeftParentheses,
    RightParentheses,
    Unit,
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn go(prev_pat: Pattern, seq: &str) -> Option<Expr> {
    let (head, tail, next) = get_head_tail_next(seq);

    let pat = match (&prev_pat, head) {
        // Start: '(' -> `(`
        (Pattern::Start, Some(c)) if parse_l_parentheses(&c) =>
            Pattern::LeftParentheses,
        // Start: [0-9] -> Digit
        (Pattern::Start, Some(c)) if parse_digit(&c).is_some() =>
            Pattern::Digit(c),
        // Start: [a-z] -> Lower
        (Pattern::Start, Some(c)) if parse_lower(&c).is_some() =>
            Pattern::Lower(c),

        // AnySeq: _ :!End -> Any
        (Pattern::AnySeq(_), Some(c)) if next != None =>
            Pattern::Any(c),
        // Lower: [0-9a-zA-Z] -> Char
        (Pattern::Lower(_), Some(c)) if parse_char(&c).is_some() =>
            Pattern::Char(c),
        // CharSeq: [0-9a-zA-Z] -> Char
        (Pattern::CharSeq(_), Some(c)) if parse_char(&c).is_some() =>
            Pattern::Char(c),
        // DigitSeq: [0-9] -> Digit
        (Pattern::DigitSeq(_), Some(c)) if parse_digit(&c).is_some() =>
            Pattern::Digit(c),
        // ')' -> `)`
        (_, Some(c)) if parse_r_parentheses(&c) =>
            Pattern::RightParentheses,
        // `(`: _ -> Any
        (Pattern::LeftParentheses, Some(c)) =>
            Pattern::Any(c),

        // ɛ -> End
        (_, None) => Pattern::End,
        // _ -> Err
        (_, Some(c)) => {
            println!("Invalid head pattern: {}", c);
            Pattern::Err
        }
    };
    let this_pat = match (prev_pat, pat) {
        // Success with Unit
        (Pattern::Unit, Pattern::End) => return Some(Expr::Unit),
        // Success with Int
        (Pattern::Int(i), Pattern::End) => return Some(Expr::Int(i)),
        // Success with EnvRef
        (Pattern::LetName(n), Pattern::End) => return Some(Expr::EnvRef(n)),

        // `(` `)` -> Unit
        (Pattern::LeftParentheses, Pattern::RightParentheses) =>
            Pattern::Unit,

        // Start Digit -> DigitSeq
        (Pattern::Start, Pattern::Digit(d)) =>
            Pattern::DigitSeq(d.to_string()),
        // DigitSeq Digit -> DigitSeq
        (Pattern::DigitSeq(a), Pattern::Digit(b)) =>
            Pattern::DigitSeq(format!("{}{}", a, b)),
        // DigitSeq End -> Int/Err
        (Pattern::DigitSeq(ds), Pattern::End) =>
            match parse_int(&ds) {
                Some((i, _)) => Pattern::Int(i),
                None => Pattern::Err
            },

        // Lower Char -> CharSeq
        (Pattern::Lower(a), Pattern::Char(b)) =>
            Pattern::CharSeq(format!("{}{}", a, b)),
        // CharSeq Char -> CharSeq
        (Pattern::CharSeq(a), Pattern::Char(b)) =>
            Pattern::CharSeq(format!("{}{}", a, b)),
        // CharSeq End -> LetName/Err
        (Pattern::CharSeq(cs), Pattern::End) =>
            match parse_let_name(&cs) {
                Some((n, _)) => Pattern::LetName(n),
                None => Pattern::Err
            },

        // `(` Any -> AnySeq
        (Pattern::LeftParentheses, Pattern::Any(c)) =>
            Pattern::AnySeq(c.to_string()),
        // AnySeq Any -> AnySeq
        (Pattern::AnySeq(a), Pattern::Any(b)) =>
            Pattern::AnySeq(format!("{}{}", a, b)),
        // AnySeq `)` -> AnySeq
        (Pattern::AnySeq(a), Pattern::RightParentheses) =>
            Pattern::AnySeq(a),
        // AnySeq End -> Expr
        (Pattern::AnySeq(seq), Pattern::End) =>
            return go(Pattern::Start, &seq),

        // Start: _ -> _
        (Pattern::Start, p) => p,

        // Can not parse
        (_, Pattern::Err) => return None,
        // Can not reduce
        (a, b) => {
            println!("Invalid reduce pattern: {}, {}", a, b);
            return None;
        }
    };

    go(this_pat, tail)
}

pub fn parse_expr(seq: &str) -> Option<Expr> {
    go(Pattern::Start, seq)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_expr() {
        use crate::parser::expr::{Expr, parse_expr};

        {
            let r = Some(Expr::Unit);
            assert_eq!(parse_expr("()"), r);
            assert_eq!(parse_expr("(())"), r);
            assert_eq!(parse_expr("((()))"), r);
        }
        {
            let r = Some(Expr::Int(123));
            assert_eq!(parse_expr("123"), r);
            assert_eq!(parse_expr("(123)"), r);
            assert_eq!(parse_expr("((123))"), r);
        }
        {
            let r = Some(Expr::EnvRef("abc".to_string()));
            assert_eq!(parse_expr("abc"), r);
            assert_eq!(parse_expr("(abc)"), r);
            assert_eq!(parse_expr("((abc))"), r);
        }
    }
}
