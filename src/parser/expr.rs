use std::{vec};
use crate::parser::char::{parse_char, parse_digit, parse_lower};
use crate::parser::get_head_tail_follow;
use crate::parser::mark::{parse_blank, parse_l_parentheses, parse_r_parentheses};
use crate::parser::name::let_name::parse_let_name;
use crate::parser::value::int::parse_int;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Expr {
    Unit,
    Int(i64),
    EnvRef(String),
    Apply(Box<Expr>, Box<Expr>),
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

    LeftParentheses,
    RightParentheses,
    Unit,

    Digit(char),
    DigitSeq(String),
    Int(i64),

    Lower(char),
    Char(char),
    CharSeq(String),
    LetName(String),

    Any(char),
    AnySeq(String),

    Blank,
    //Lhs(String),
    //Rhs(String),
}

fn go(stack: &Vec<Pattern>, seq: &str) -> Option<Expr> {
    let (head, tail, follow) = get_head_tail_follow(seq);

    let follow = match follow {
        None => Pattern::End,
        Some(' ') => Pattern::Blank,
        Some(c) => Pattern::Any(c),
    };

    let move_in = match (&stack[..], head, &follow) {
        // Start: '(' -> `(`
        ([Pattern::Start], Some(c), _) if parse_l_parentheses(&c) =>
            Pattern::LeftParentheses,
        // Start: [0-9] -> Digit
        ([Pattern::Start], Some(c), _) if parse_digit(&c).is_some() =>
            Pattern::Digit(c),
        // Start: [a-z] -> Lower
        ([Pattern::Start], Some(c), _) if parse_lower(&c).is_some() =>
            Pattern::Lower(c),

        // ' ' -> End
        (_, Some(c), _) if parse_blank(&c) =>
            Pattern::End,
        // AnySeq: _ :!End -> Any
        ([Pattern::AnySeq(_)], Some(c), np) if np != &Pattern::End =>
            Pattern::Any(c),
        // Lower: [0-9a-zA-Z] -> Char
        ([Pattern::Lower(_)], Some(c), _) if parse_char(&c).is_some() =>
            Pattern::Char(c),
        // CharSeq: [0-9a-zA-Z] -> Char
        ([Pattern::CharSeq(_)], Some(c), _) if parse_char(&c).is_some() =>
            Pattern::Char(c),
        // DigitSeq: [0-9] -> Digit
        ([Pattern::DigitSeq(_)], Some(c), _) if parse_digit(&c).is_some() =>
            Pattern::Digit(c),
        // ')' -> `)`
        (_, Some(c), _) if parse_r_parentheses(&c) =>
            Pattern::RightParentheses,
        // `(`: _ -> Any
        ([Pattern::LeftParentheses], Some(c), _) =>
            Pattern::Any(c),

        // ɛ -> End
        (_, None, _) => Pattern::End,
        // _ -> Err
        (_, Some(c), _) => {
            println!("Invalid head pattern: {}", c);
            Pattern::Err
        }
    };

    let reduced_stack = match (&stack[..], move_in, follow) {
        // Success with Unit
        ([Pattern::Unit], Pattern::End, _) => return Some(Expr::Unit),
        // Success with Int
        ([Pattern::Int(i)], Pattern::End, _) => return Some(Expr::Int(*i)),
        // Success with EnvRef
        ([Pattern::LetName(n)], Pattern::End, _) =>
            return Some(Expr::EnvRef(n.to_string())),

        // `(` `)` -> Unit
        ([Pattern::LeftParentheses], Pattern::RightParentheses, _) =>
            vec![Pattern::Unit],

        // Start Digit -> DigitSeq
        ([Pattern::Start], Pattern::Digit(d), _) =>
            vec![Pattern::DigitSeq(d.to_string())],
        // DigitSeq Digit -> DigitSeq
        ([Pattern::DigitSeq(a)], Pattern::Digit(b), _) =>
            vec![Pattern::DigitSeq(format!("{}{}", a, b))],
        // DigitSeq End -> Int/Err
        ([Pattern::DigitSeq(ds)], Pattern::End, _) =>
            match parse_int(&ds) {
                Some(i) => vec![Pattern::Int(i)],
                None => vec![Pattern::Err]
            },

        // Lower Char -> CharSeq
        ([Pattern::Lower(a)], Pattern::Char(b), _) =>
            vec![Pattern::CharSeq(format!("{}{}", a, b))],
        // CharSeq Char -> CharSeq
        ([Pattern::CharSeq(a)], Pattern::Char(b), _) =>
            vec![Pattern::CharSeq(format!("{}{}", a, b))],
        // CharSeq End -> LetName/Err
        ([Pattern::CharSeq(cs)], Pattern::End, _) =>
            match parse_let_name(&cs) {
                Some(n) => vec![Pattern::LetName(n)],
                None => vec![Pattern::Err]
            },

        // `(` Any -> AnySeq
        ([Pattern::LeftParentheses], Pattern::Any(c), _) =>
            vec![Pattern::AnySeq(c.to_string())],
        // AnySeq Any -> AnySeq
        ([Pattern::AnySeq(a)], Pattern::Any(b), _) =>
            vec![Pattern::AnySeq(format!("{}{}", a, b))],
        // AnySeq `)` -> AnySeq
        ([Pattern::AnySeq(a)], Pattern::RightParentheses, _) =>
            vec![Pattern::AnySeq(a.to_string())],
        // AnySeq End -> Expr
        ([Pattern::AnySeq(seq)], Pattern::End, _) =>
            return go(&vec![Pattern::Start], &seq),

        // Start: _ -> _
        ([Pattern::Start], p, _) => vec![p],

        // Can not parse
        (_, Pattern::Err, _) => return None,
        // Can not reduce
        (_, b, _) => {
            println!("Invalid reduce pattern: {:?}, {:?}", stack, b);
            return None;
        }
    };

    go(&reduced_stack, tail)
}

pub fn parse_expr(seq: &str) -> Option<Expr> {
    go(&vec![Pattern::Start], seq)
}

#[cfg(test)]
mod tests {
    use crate::parser::expr::{Expr, parse_expr};

    #[test]
    fn test_parse_expr_unit() {
        let r = Some(Expr::Unit);
        assert_eq!(parse_expr("()"), r);
        assert_eq!(parse_expr("(())"), r);
        assert_eq!(parse_expr("((()))"), r);
    }

    #[test]
    fn test_parse_expr_int() {
        let r = Some(Expr::Int(123));
        assert_eq!(parse_expr("123"), r);
        assert_eq!(parse_expr("(123)"), r);
        assert_eq!(parse_expr("((123))"), r);
    }

    #[test]
    fn test_parse_expr_env_ref() {
        let r = Some(Expr::EnvRef("abc".to_string()));
        assert_eq!(parse_expr("abc"), r);
        assert_eq!(parse_expr("(abc)"), r);
        assert_eq!(parse_expr("((abc))"), r);
    }

    #[test]
    fn test_parse_expr_apply() {
        // Unit Int
        let r = Some(Expr::Apply(
            Box::new(Expr::Unit),
            Box::new(Expr::Int(123)),
        ));
        assert_eq!(parse_expr("() 123"), r);
        assert_eq!(parse_expr("(()) (123)"), r);
        assert_eq!(parse_expr("((())) ((123))"), r);
        assert_eq!(parse_expr("(((())) ((123)))"), r);
        assert_eq!(parse_expr("((((())) ((123))))"), r);

        //// EnvRef Int
        //let r = Some(Expr::Apply(
        //    Box::new(Expr::EnvRef("abc".to_string())),
        //    Box::new(Expr::Int(123)),
        //));
        //assert_eq!(parse_expr("abc 123"), r);
        //assert_eq!(parse_expr("(abc) (123)"), r);
        //assert_eq!(parse_expr("((abc)) ((123))"), r);
        //assert_eq!(parse_expr("(((abc)) ((123)))"), r);
        //assert_eq!(parse_expr("((((abc)) ((123))))"), r);

        //// EnvRef Unit
        //let r = Some(Expr::Apply(
        //    Box::new(Expr::EnvRef("abc".to_string())),
        //    Box::new(Expr::Unit),
        //));
        //assert_eq!(parse_expr("abc ()"), r);
        //assert_eq!(parse_expr("(abc) (())"), r);
        //assert_eq!(parse_expr("((abc)) ((()))"), r);
        //assert_eq!(parse_expr("(((abc)) ((())))"), r);
        //assert_eq!(parse_expr("((((abc)) ((()))))"), r);

        //// EnvRef (EnvRef Unit)
        //let r = Some(Expr::Apply(
        //    Box::new(Expr::EnvRef("abc".to_string())),
        //    Box::new(r.unwrap()),
        //));
        //assert_eq!(parse_expr("abc (abc ())"), r);
        //assert_eq!(parse_expr("(abc) ((abc ()))"), r);
        //assert_eq!(parse_expr("((abc)) (((abc ())))"), r);
        //assert_eq!(parse_expr("(((abc)) (((abc ()))))"), r);
        //assert_eq!(parse_expr("((((abc)) (((abc ())))))"), r);

        //// EnvRef (EnvRef (EnvRef Unit))
        //let r = Some(Expr::Apply(
        //    Box::new(Expr::EnvRef("abc".to_string())),
        //    Box::new(r.unwrap()),
        //));
        //assert_eq!(parse_expr("abc (abc (abc ()))"), r);
        //assert_eq!(parse_expr("(abc) ((abc (abc ())))"), r);
        //assert_eq!(parse_expr("((abc)) (((abc (abc ()))))"), r);
        //assert_eq!(parse_expr("(((abc)) (((abc (abc ())))))"), r);
        //assert_eq!(parse_expr("((((abc)) (((abc (abc ()))))))"), r);
    }
}
