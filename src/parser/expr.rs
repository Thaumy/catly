use std::{vec};
use crate::parser::char::{parse_char, parse_digit};
use crate::parser::{Ext, get_head_tail_follow};
use crate::parser::keyword::{parse_else, parse_if, parse_then};
use crate::parser::mark::{parse_l_parentheses, parse_r_parentheses};
use crate::parser::name::let_name::parse_let_name;
use crate::parser::value::int::parse_int;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Expr {
    Unit,
    Int(i64),
    EnvRef(String),
    Apply(Box<Expr>, Box<Expr>),
    Cond(Box<Expr>, Box<Expr>, Box<Expr>),
    //Closure,
    //Struct,
    //Match,
}

#[derive(Debug)
#[derive(Clone)]
#[derive(PartialEq)]
enum Pat {
    Start,
    End,
    Err,

    Any(char),

    LeftParentheses,
    RightParentheses,
    Unit,//Expr::Unit

    Digit(char),
    DigitSeq(String),
    Int(i64),//Expr::Int

    Char(char),
    CharSeq(String),
    LetName(String),//Expr::EnvRef

    Blank,
    Apply(Box<Pat>, Box<Pat>),

    If,
    Then,
    Else,
    Cond(Box<Pat>, Box<Pat>, Box<Pat>),//Expr::Cond
}

fn move_in(stack: &Vec<Pat>, head: Option<char>) -> Pat {
    match (&stack[..], head) {

        // DigitSeq: [0-9] -> Digit
        ([.., Pat::DigitSeq(_)], Some(c)) if parse_digit(&c).is_some() =>
            Pat::Digit(c),
        // [0-9] -> Digit
        (_, Some(c)) if parse_digit(&c).is_some() =>
            Pat::Digit(c),

        // CharSeq: [0-9a-zA-Z] -> Char
        ([.., Pat::CharSeq(_)], Some(c)) if parse_char(&c).is_some() =>
            Pat::Char(c),
        // [0-9a-zA-Z] -> Char
        (_, Some(c)) if parse_char(&c).is_some() =>
            Pat::Char(c),

        // ' ' -> Blank
        (_, Some(' ')) => Pat::Blank,
        // '(' -> `(`
        (_, Some(c)) if parse_l_parentheses(&c) =>
            Pat::LeftParentheses,
        // ')' -> `)`
        (_, Some(c)) if parse_r_parentheses(&c) =>
            Pat::RightParentheses,

        // ɛ -> End
        (_, None) => Pat::End,
        // _ -> Err
        (_, Some(c)) => {
            println!("Invalid head Pat: {}", c);
            Pat::Err
        }
    }
}

fn reduce_stack(stack: &Vec<Pat>, follow_pat: &Pat) -> Vec<Pat> {
    let reduced_stack = match (&stack[..], follow_pat) {
        // Success with Unit
        ([Pat::Start, Pat::Unit, Pat::End], _) => return vec![Pat::Unit],
        // Success with Int
        ([Pat::Start, Pat::Int(i), Pat::End], _) => return vec![Pat::Int(*i)],
        // Success with EnvRef
        ([Pat::Start, Pat::LetName(n), Pat::End], _) =>
            return vec![Pat::LetName(n.to_string())],
        // Success with Apply
        ([Pat::Start, Pat::Apply(lhs, rhs), Pat::End], _) =>
            return vec![Pat::Apply(
                lhs.clone(),
                rhs.clone(),
            )],
        // Success with Cond
        ([Pat::Start, Pat::Cond(a, b, c), Pat::End], _) =>
            return vec![Pat::Cond(
                a.clone(),
                b.clone(),
                c.clone(),
            )],

        // CharSeq("if") :Blank -> If
        ([.., Pat::CharSeq(cs)], Pat::Blank) if parse_if(cs) =>
            stack.reduce_to_new(1, Pat::If),
        // CharSeq("then") :Blank -> Then
        ([.., Pat::CharSeq(cs)], Pat::Blank) if parse_then(cs) =>
            stack.reduce_to_new(1, Pat::Then),
        // CharSeq("else") :Blank -> Else
        ([.., Pat::CharSeq(cs) ], Pat::Blank) if parse_else(cs) =>
            stack.reduce_to_new(1, Pat::Else),
        // If Blank Expr Blank Then Blank Expr Blank Else Blank Expr -> Cond
        ([..,
        Pat::If, Pat::Blank, a, Pat::Blank,
        Pat::Then, Pat::Blank, b, Pat::Blank,
        Pat::Else, Pat::Blank, c
        ], _)
        if match (a, b, c) {
            (Pat::Unit | Pat::Int(_) | Pat::LetName(_) | Pat::Apply(_, _) | Pat::Cond(_, _, _),
                Pat::Unit | Pat::Int(_) | Pat::LetName(_) | Pat::Apply(_, _) | Pat::Cond(_, _, _),
                Pat::Unit | Pat::Int(_) | Pat::LetName(_) | Pat::Apply(_, _) | Pat::Cond(_, _, _),
            ) => true,
            _ => false
        } =>
            stack.reduce_to_new(11, Pat::Cond(
                Box::new(a.clone()),
                Box::new(b.clone()),
                Box::new(c.clone()),
            )),

        // `(` `)` -> Unit
        ([.., Pat::LeftParentheses, Pat::RightParentheses], _) =>
            stack.reduce_to_new(2, Pat::Unit),

        // `(` _ `)` -> _
        ([.., Pat::LeftParentheses, p, Pat::RightParentheses], _) =>
            stack.reduce_to_new(3, p.clone()),

        // DigitSeq Digit -> DigitSeq
        ([.., Pat::DigitSeq(ds), Pat::Digit(d)], _) =>
            stack.reduce_to_new(2, Pat::DigitSeq(format!("{}{}", ds, d))),
        // DigitSeq :Digit -> DigitSeq
        ([.., Pat::DigitSeq(_)], Pat::Digit(_)) =>
            return stack.clone(),
        // DigitSeq :!Digit -> Int|Err
        ([.., Pat::DigitSeq(ds)], _) => {
            let top = match parse_int(ds) {
                Some(i) => Pat::Int(i),
                None => Pat::Err
            };
            stack.reduce_to_new(1, top)
        }
        // Digit :Digit -> DigitSeq
        ([.., Pat::Digit(d)], Pat::Digit(_)) =>
            stack.reduce_to_new(1, Pat::DigitSeq(d.to_string())),
        // Digit :!Digit -> Int|Err
        ([.., Pat::Digit(d)], _) => {
            let top = match parse_int(&d.to_string()) {
                Some(i) => Pat::Int(i),
                None => Pat::Err
            };
            stack.reduce_to_new(1, top)
        }

        // CharSeq Char -> CharSeq
        ([.., Pat::CharSeq(cs), Pat::Char(c)], _) =>
            stack.reduce_to_new(2, Pat::CharSeq(format!("{}{}", cs, c))),
        // CharSeq :Char -> CharSeq
        ([.., Pat::CharSeq(_)], Pat::Char(_)) =>
            return stack.clone(),
        // CharSeq :!Char-> LetName|Err
        ([.., Pat::CharSeq(cs)], _) => {
            let top = match parse_let_name(cs) {
                Some(n) => Pat::LetName(n),
                None => Pat::Err
            };
            stack.reduce_to_new(1, top)
        }
        // Char :Char -> CharSeq
        ([.., Pat::Char(c)], Pat::Char(_) | Pat::Digit(_)) =>
            stack.reduce_to_new(1, Pat::CharSeq(c.to_string())),
        // Char :!Char -> LetName|Err
        ([.., Pat::Char(c)], _) => {
            let top = match parse_let_name(&c.to_string()) {
                Some(n) => Pat::LetName(n),
                None => Pat::Err
            };
            stack.reduce_to_new(1, top)
        }

        // _ Blank Expr -> Apply
        ([.., lhs, Pat::Blank, rhs], _)
        if match (lhs, rhs) {
            (
                Pat::Unit | Pat::Int(_) | Pat::LetName(_) | Pat::Apply(_, _) | Pat::Cond(_, _, _),
                Pat::Unit | Pat::Int(_) | Pat::LetName(_) | Pat::Apply(_, _) | Pat::Cond(_, _, _)
            ) => true,
            _ => false
        } => {
            let top = Pat::Apply(
                Box::new(lhs.clone()),
                Box::new(rhs.clone()),
            );
            stack.reduce_to_new(3, top)
        }

        // Can not parse
        ([.., Pat::Err], _) => return vec![Pat::Err],
        // Can not reduce
        ([.., Pat::End], _) => {
            println!("Reduction failed: {:?}", stack);
            return vec![Pat::Err];
        }
        // keep move in
        _ => return stack.clone()
    };

    println!("Reduce to: {:?}", reduced_stack);

    reduce_stack(&reduced_stack, follow_pat)
}

fn go(stack: &Vec<Pat>, seq: &str) -> Pat {
    let (head, tail, follow) = get_head_tail_follow(seq);

    let follow_pat = match follow {
        None => Pat::End,
        Some(' ') => Pat::Blank,
        Some(c) if parse_digit(&c).is_some() => Pat::Digit(c),
        Some(c) if parse_char(&c).is_some() => Pat::Char(c),
        Some(c) => Pat::Any(c),
    };

    let stack = stack.push_to_new(move_in(stack, head));
    println!("Move in result: {:?} follow: {:?}", stack, follow_pat);

    let reduced_stack = reduce_stack(&stack, &follow_pat);

    match (&reduced_stack[..], follow_pat) {
        ([p], Pat::End) => {
            let r = p.clone();
            println!("Success with: {:?}", r);
            return r;
        }
        _ => go(&reduced_stack, tail)
    }
}

pub fn parse_expr(seq: &str) -> Option<Expr> {
    fn case(pat: Pat) -> Option<Expr> {
        let r = match pat {
            Pat::Unit => Expr::Unit,
            Pat::Int(i) => Expr::Int(i),
            Pat::LetName(n) => Expr::EnvRef(n),
            Pat::Apply(l, r) =>
                match (case(*l), case(*r)) {
                    (Some(l), Some(r)) =>
                        Expr::Apply(
                            Box::new(l),
                            Box::new(r),
                        ),
                    _ => return None
                }
            Pat::Cond(a, b, c) =>
                match (case(*a), case(*b), case(*c)) {
                    (Some(a), Some(b), Some(c)) =>
                        Expr::Cond(
                            Box::new(a),
                            Box::new(b),
                            Box::new(c),
                        ),
                    _ => return None
                }
            _ => return None
        };
        Some(r)
    }

    println!("\nParsing seq: {:?}", seq);
    case(go(&vec![Pat::Start], seq))
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
    fn test_parse_expr_apply_part1() {
        // Apply(Unit, Int)
        let r = Some(Expr::Apply(
            Box::new(Expr::Unit),
            Box::new(Expr::Int(123)),
        ));
        assert_eq!(parse_expr("() 123"), r);
        assert_eq!(parse_expr("(()) (123)"), r);
        assert_eq!(parse_expr("((())) ((123))"), r);
        assert_eq!(parse_expr("(((())) ((123)))"), r);
        assert_eq!(parse_expr("((((())) ((123))))"), r);
    }

    #[test]
    fn test_parse_expr_apply_part2() {
        // Apply(EnvRef, Int)
        let r = Some(Expr::Apply(
            Box::new(Expr::EnvRef("abc".to_string())),
            Box::new(Expr::Int(123)),
        ));
        assert_eq!(parse_expr("abc 123"), r);
        assert_eq!(parse_expr("(abc) (123)"), r);
        assert_eq!(parse_expr("((abc)) ((123))"), r);
        assert_eq!(parse_expr("(((abc)) ((123)))"), r);
        assert_eq!(parse_expr("((((abc)) ((123))))"), r);
    }

    #[test]
    fn test_parse_expr_apply_part3() {
        // Apply(EnvRef, Unit)
        let r = Some(Expr::Apply(
            Box::new(Expr::EnvRef("abc".to_string())),
            Box::new(Expr::Unit),
        ));
        assert_eq!(parse_expr("abc ()"), r);
        assert_eq!(parse_expr("(abc) (())"), r);
        assert_eq!(parse_expr("((abc)) ((()))"), r);
        assert_eq!(parse_expr("(((abc)) ((())))"), r);
        assert_eq!(parse_expr("((((abc)) ((()))))"), r);
    }

    #[test]
    fn test_parse_expr_apply_part4() {
        // Apply(EnvRef, Apply(EnvRef, Unit))
        let r = Some(Expr::Apply(
            Box::new(Expr::EnvRef("abc".to_string())),
            Box::new(Expr::Apply(
                Box::new(Expr::EnvRef("abc".to_string())),
                Box::new(Expr::Unit),
            )),
        ));
        assert_eq!(parse_expr("abc (abc ())"), r);
        assert_eq!(parse_expr("(abc) ((abc ()))"), r);
        assert_eq!(parse_expr("((abc)) (((abc ())))"), r);
        assert_eq!(parse_expr("(((abc)) (((abc ()))))"), r);
        assert_eq!(parse_expr("((((abc)) (((abc ())))))"), r);
    }

    #[test]
    fn test_parse_expr_apply_part5() {
        // Apply(EnvRef, Apply(EnvRef, Apply(EnvRef, Unit)))
        let r = Some(Expr::Apply(
            Box::new(Expr::EnvRef("abc".to_string())),
            Box::new(Expr::Apply(
                Box::new(Expr::EnvRef("abc".to_string())),
                Box::new(Expr::Apply(
                    Box::new(Expr::EnvRef("abc".to_string())),
                    Box::new(Expr::Unit),
                )),
            )),
        ));
        assert_eq!(parse_expr("abc (abc (abc ()))"), r);
        assert_eq!(parse_expr("(abc) ((abc (abc ())))"), r);
        assert_eq!(parse_expr("((abc)) (((abc (abc ()))))"), r);
        assert_eq!(parse_expr("(((abc)) (((abc (abc ())))))"), r);
        assert_eq!(parse_expr("((((abc)) (((abc (abc ()))))))"), r);
    }

    #[test]
    fn test_parse_expr_cond_part1() {
        // Cond(EnvRef, Int, Unit)
        let r = Some(Expr::Cond(
            Box::new(Expr::EnvRef("abc".to_string())),
            Box::new(Expr::Int(123)),
            Box::new(Expr::Unit),
        ));
        assert_eq!(parse_expr("if abc then 123 else ()"), r);
        assert_eq!(parse_expr("if ((abc)) then ((123)) else ((()))"), r);
        assert_eq!(parse_expr("(if (((abc))) then (((123))) else (((()))))"), r);
        assert_eq!(parse_expr("(((if (((abc))) then (((123))) else (((()))))))"), r);
    }

    #[test]
    fn test_parse_expr_cond_part2() {
        // Cond(a, a, a)
        // while: a = Cond(EnvRef, Apply(Int, Unit), Int)
        let e = Expr::Cond(
            Box::new(Expr::EnvRef("abc".to_string())),
            Box::new(Expr::Apply(
                Box::new(Expr::Int(123)),
                Box::new(Expr::Unit))
            ),
            Box::new(Expr::Int(456)),
        );
        let r = Some(Expr::Cond(
            Box::new(e.clone()),
            Box::new(e.clone()),
            Box::new(e.clone()),
        ));

        let e = "if abc then 123 () else 456";
        let seq = &format!("if {} then {} else {}", e, e, e);
        assert_eq!(parse_expr(seq), r);
        let e = "if abc then (123 ()) else 456";
        let seq = &format!("if {} then {} else {}", e, e, e);
        assert_eq!(parse_expr(seq), r);
        let e = "(((if ((abc)) then ((123 ())) else ((456)))))";
        let seq = &format!("if {} then {} else {}", e, e, e);
        assert_eq!(parse_expr(seq), r);
    }

    #[test]
    fn test_parse_expr_cond_part3() {
        // Cond(b, b, b)
        // while: a = Cond(Apply(Int, Unit), Int, EnvRef)
        // while: b = Cond(a, a, a)
        let a = Expr::Cond(
            Box::new(Expr::Apply(
                Box::new(Expr::Int(123)),
                Box::new(Expr::Unit),
            )),
            Box::new(Expr::Int(123)),
            Box::new(Expr::EnvRef("abc".to_string())),
        );
        let b = Expr::Cond(
            Box::new(a.clone()),
            Box::new(a.clone()),
            Box::new(a.clone()),
        );
        let r = Some(Expr::Cond(
            Box::new(b.clone()),
            Box::new(b.clone()),
            Box::new(b.clone()),
        ));

        let a = "if 123 () then 123 else abc";
        let b = &format!("if {} then {} else {}", a, a, a);
        let seq = &format!("if {} then {} else {}", b, b, b);
        assert_eq!(parse_expr(seq), r);
        let a = "(((if (((123 ()))) then (((123))) else (((abc))))))";
        let b = &format!("(((if {} then {} else {})))", a, a, a);
        let seq = &format!("if {} then {} else {}", b, b, b);
        assert_eq!(parse_expr(seq), r);
    }

    #[test]
    fn test_parse_expr_cond_part4() {
        // Cond(b, b, b)
        // while: a = Cond(Apply(Int, Unit), Int, EnvRef)
        // while: b = Cond(a, a, a)
        let a = Expr::Cond(
            Box::new(Expr::Apply(
                Box::new(Expr::Int(123)),
                Box::new(Expr::Unit),
            )),
            Box::new(Expr::Int(123)),
            Box::new(Expr::EnvRef("abc".to_string())),
        );
        let b = Expr::Cond(
            Box::new(a.clone()),
            Box::new(a.clone()),
            Box::new(a.clone()),
        );
        let r = Some(Expr::Cond(
            Box::new(b.clone()),
            Box::new(b.clone()),
            Box::new(b.clone()),
        ));

        let a = "(((if (((123 ()))) then (((123))) else (((abc))))))";
        let b = &format!("(((if ((({}))) then ((({}))) else {})))", a, a, a);
        let seq = &format!("(((if ((({}))) then {} else ((({}))))))", b, b, b);
        assert_eq!(parse_expr(seq), r);
    }
}
