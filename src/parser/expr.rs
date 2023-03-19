mod follow_pat;
mod pat;

use std::{vec};
use crate::parser::char::{parse_char, parse_digit};
use crate::parser::{VecExt, vec_get_head_tail_follow};
use crate::parser::expr::follow_pat::{FollowPat, parse_follow_pat};
use crate::parser::expr::pat::Pat;
use crate::parser::keyword::{Keyword};
use crate::parser::name::let_name::parse_let_name;
use crate::parser::preprocess::blank::preprocess_blank;
use crate::parser::preprocess::comment::preprocess_comment;
use crate::parser::preprocess::keyword::{Either, preprocess_keyword};
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
    Closure(String, Box<Expr>),
    Struct(Vec<(String, Expr)>),
    Discard,
    Match(Box<Expr>, Vec<(Expr, Expr)>),
    Let(String, Box<Expr>, Box<Expr>),
}

fn move_in(stack: &Vec<Pat>, head: Option<Either<char, Keyword>>) -> Pat {
    match head {
        Some(Either::L(o)) => match (&stack[..], o) {
            // DigitSeq: [0-9] -> Digit
            ([.., Pat::DigitSeq(_)], c) if parse_digit(&c).is_some() =>
                Pat::Digit(c),
            // [0-9] -> Digit
            (_, c) if parse_digit(&c).is_some() =>
                Pat::Digit(c),

            // CharSeq: [0-9a-zA-Z] -> Char
            ([.., Pat::CharSeq(_)], c) if parse_char(&c).is_some() =>
                Pat::Char(c),
            // [0-9a-zA-Z] -> Char
            (_, c) if parse_char(&c).is_some() =>
                Pat::Char(c),

            // ' ' -> Blank
            (_, ' ') => Pat::Blank,
            // '(' -> `(`
            (_, '(') => Pat::Mark('('),
            // ')' -> `)`
            (_, ')') => Pat::Mark(')'),

            // '-' -> `-`
            (_, '-') => Pat::Mark('-'),
            // '>' -> `>`
            (_, '>') => Pat::Mark('>'),

            // '{' -> `{`
            (_, '{') => Pat::Mark('{'),
            // '}' -> `}`
            (_, '}') => Pat::Mark('}'),
            // '=' -> `=`
            (_, '=') => Pat::Mark('='),
            // ',' -> `,`
            (_, ',') => Pat::Mark(','),

            // '|' -> `|`
            (_, '|') => Pat::Mark('|'),
            // '_' -> Discard
            (_, '_') => Pat::Discard,

            // _ -> Err
            (_, c) => {
                println!("Invalid head Pat: {}", c);
                Pat::Err
            }
        }

        Some(Either::R(kw)) => kw.into(),

        // ɛ -> End
        None => Pat::End,
    }
}

fn reduce_stack(stack: &Vec<Pat>, follow_pat: &FollowPat) -> Vec<Pat> {
    let reduced_stack = match (&stack[..], follow_pat) {
        // Success
        ([Pat::Start, p, Pat::End], FollowPat::End) => return vec![p.clone()],

        // `(` `)` -> Unit
        ([.., Pat::Mark('('), Pat::Mark(')')], _) =>
            stack.reduce_to_new(2, Pat::Unit),

        // `(` _ `)` -> _
        ([.., Pat::Mark('('), p, Pat::Mark(')')], _) =>
            stack.reduce_to_new(3, p.clone()),

        // KwIf Blank Expr Blank KwThen Blank Expr Blank KwElse Blank Expr -> Cond
        ([..,
        Pat::KwIf, Pat::Blank, a, Pat::Blank,
        Pat::KwThen, Pat::Blank, b, Pat::Blank,
        Pat::KwElse, Pat::Blank, c
        ], _)
        if a.is_expr() && b.is_expr() && c.is_expr() =>
            stack.reduce_to_new(11, Pat::Cond(
                Box::new(a.clone()),
                Box::new(b.clone()),
                Box::new(c.clone()),
            )),

        // DigitSeq Digit -> DigitSeq
        ([.., Pat::DigitSeq(ds), Pat::Digit(d)], _) =>
            stack.reduce_to_new(2, Pat::DigitSeq(format!("{}{}", ds, d))),
        // DigitSeq :Digit -> DigitSeq
        ([.., Pat::DigitSeq(_)], FollowPat::Digit(_)) =>
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
        ([.., Pat::Digit(d)], FollowPat::Digit(_)) =>
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
        ([.., Pat::CharSeq(_)], FollowPat::Letter(_) | FollowPat::Digit(_)) =>
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
        ([.., Pat::Char(c)], FollowPat::Letter(_) | FollowPat::Digit(_)) =>
            stack.reduce_to_new(1, Pat::CharSeq(c.to_string())),
        // Char :!Char -> LetName|Err
        ([.., Pat::Char(c)], _) => {
            let top = match parse_let_name(&c.to_string()) {
                Some(n) => Pat::LetName(n),
                None => Pat::Err
            };
            stack.reduce_to_new(1, top)
        }

        // `-` `>` -> Arrow
        ([.., Pat::Mark('-'), Pat::Mark('>')], _) =>
            stack.reduce_to_new(2, Pat::Arrow),
        // LetName Blank Arrow Blank -> ClosurePara
        ([.., Pat::LetName(n), Pat::Blank, Pat::Arrow, Pat::Blank], _) => {
            let top = Pat::ClosurePara(n.to_string());
            stack.reduce_to_new(4, top)
        }
        // ClosurePara Expr :!Blank -> Closure
        /* TODO: 此产生式要求当 Closure 具备如下形式时:
                 x -> y -> z }
                 Closure 必须被括号环绕:
                 (x -> y -> z) }
                 否则将无法归约 */
        ([.., Pat::ClosurePara(n), p], follow_pat)
        if follow_pat.not_blank() && p.is_expr() => {
            let top = Pat::Closure(
                n.to_string(),
                Box::new(p.clone()),
            );
            stack.reduce_to_new(2, top)
        }

        // Blank LetName Blank `=` Blank Expr `,` -> Assign
        ([.., Pat::Blank,
        Pat::LetName(cs), Pat::Blank, Pat::Mark('='), Pat::Blank,
        p, Pat::Mark(',')], _
        )
        if p.is_expr() => {
            let top = Pat::Assign(cs.clone(), Box::new(p.clone()));
            stack.reduce_to_new(7, top)
        }
        // Blank LetName Blank `=` Blank Expr Blank :`}`-> Assign
        ([.., Pat::Blank,
        Pat::LetName(cs), Pat::Blank, Pat::Mark('='), Pat::Blank,
        p, Pat::Blank], FollowPat::Mark('}')
        )
        if p.is_expr() => {
            let top = Pat::Assign(cs.clone(), Box::new(p.clone()));
            stack.reduce_to_new(7, top)
        }
        // Assign Assign -> AssignSeq
        ([..,
        Pat::Assign(a_n, a_v),
        Pat::Assign(b_n, b_v)], _
        ) => {
            let top = Pat::AssignSeq(vec![
                (a_n.to_string(), *a_v.clone()),
                (b_n.to_string(), *b_v.clone()),
            ]);
            stack.reduce_to_new(2, top)
        }
        // AssignSeq Assign -> AssignSeq
        ([..,
        Pat::AssignSeq(a_seq),
        Pat::Assign(n, v)], _
        ) => {
            let top = Pat::AssignSeq(
                a_seq.push_to_new((n.clone(), *v.clone()))
            );
            stack.reduce_to_new(2, top)
        }
        // `{` AssignSeq `}` -> Struct
        ([..,
        Pat::Mark('{'),
        Pat::AssignSeq(a_seq),
        Pat::Mark('}')], _
        ) => {
            let top = Pat::Struct(a_seq.clone());
            stack.reduce_to_new(3, top)
        }
        // `{` Assign `}` -> Struct
        ([..,
        Pat::Mark('{'),
        Pat::Assign(n, v),
        Pat::Mark('}')], _
        ) => {
            let top = Pat::Struct(vec![(n.to_string(), *v.clone())]);
            stack.reduce_to_new(3, top)
        }

        // KwMatch Blank Expr Blank KwWith -> MatchHead
        ([.., Pat::KwMatch, Pat::Blank, p, Pat::Blank, Pat::KwWith], _)
        if p.is_expr() => {
            let top = Pat::MatchHead(Box::new(p.clone()));
            stack.reduce_to_new(5, top)
        }
        // `|` Blank Expr Blank Arrow -> CaseHead
        ([..,
        Pat::Mark('|'), Pat::Blank,
        p, Pat::Blank, Pat::Arrow], _
        )
        if p.is_expr() => {
            let top = Pat::CaseHead(Box::new(p.clone()));
            stack.reduce_to_new(5, top)
        }
        // CaseHead Blank Expr Blank :VerticalBar -> Case
        ([..,
        Pat::CaseHead(e), Pat::Blank,
        p, Pat::Blank ], FollowPat::Mark('|')
        )
        if p.is_expr() => {
            let top = Pat::Case(
                e.clone(),
                Box::new(p.clone()),
            );
            stack.reduce_to_new(4, top)
        }
        // CaseHead Blank Expr :!Blank -> Case
        ([..,
        Pat::CaseHead(e), Pat::Blank,
        p  ], follow_pat
        )
        if follow_pat.not_blank() && p.is_expr() => {
            let top = Pat::Case(
                e.clone(),
                Box::new(p.clone()),
            );
            stack.reduce_to_new(3, top)
        }
        // Case Case -> CaseSeq
        ([..,
        Pat::Case(a_case, a_then),
        Pat::Case(b_case, b_then) ], _
        ) => {
            let top = Pat::CaseSeq(vec![
                (*a_case.clone(), *a_then.clone()),
                (*b_case.clone(), *b_then.clone()),
            ]);
            stack.reduce_to_new(2, top)
        }
        // CaseSeq Case -> CaseSeq
        ([..,
        Pat::CaseSeq(vec),
        Pat::Case(case, then) ], _
        ) => {
            let top = Pat::CaseSeq(
                vec.push_to_new((*case.clone(), *then.clone()))
            );
            stack.reduce_to_new(2, top)
        }
        // MatchHead Case :!(Blank|`|`) -> Match
        ([..,
        Pat::MatchHead(h_e),
        Pat::Case(case, then) ], follow_pat
        )
        if match follow_pat {
            FollowPat::Blank | FollowPat::Mark('|') => false,
            _ => true
        } => {
            let top = Pat::Match(
                h_e.clone(),
                vec![((*case.clone(), *then.clone()))],
            );
            stack.reduce_to_new(2, top)
        }
        // MatchHead CaseSeq :!(Blank|`|`) -> Match
        ([..,
        Pat::MatchHead(h_e),
        Pat::CaseSeq(vec) ], follow_pat
        )
        if match follow_pat {
            FollowPat::Blank | FollowPat::Mark('|') => false,
            _ => true
        } => {
            let top = Pat::Match(
                h_e.clone(),
                vec.clone(),
            );
            stack.reduce_to_new(2, top)
        }

        // Expr Blank Expr -> Apply
        ([.., lhs, Pat::Blank, rhs], _)
        if lhs.is_expr() && rhs.is_expr() => {
            let top = Pat::Apply(
                Box::new(lhs.clone()),
                Box::new(rhs.clone()),
            );
            stack.reduce_to_new(3, top)
        }

        // Blank LetName Blank `=` Blank Expr Blank :KwIn -> Assign
        ([.., Pat::Blank,
        Pat::LetName(cs), Pat::Blank, Pat::Mark('='), Pat::Blank,
        p, Pat::Blank], FollowPat::Keyword(Pat::KwIn)
        )
        if p.is_expr() => {
            let top = Pat::Assign(cs.clone(), Box::new(p.clone()));
            stack.reduce_to_new(7, top)
        }
        // KwLet AssignSeq KwIn Blank Expr :!Blank -> Let
        ([.., Pat::KwLet,
        Pat::AssignSeq(a_seq), Pat::KwIn, Pat::Blank,
        p], follow_pat)
        if follow_pat.not_blank() && p.is_expr() => {
            type F = fn(Pat, &(String, Pat)) -> Pat;
            let f: F = |acc, (n, e)| Pat::Let(
                n.to_string(),
                Box::new(e.clone()),
                Box::new(acc),
            );
            let top = a_seq
                .iter()
                .rev()
                .fold(p.clone(), f);
            stack.reduce_to_new(5, top)
        }
        // KwLet Assign KwIn Blank Expr :!Blank -> Let
        ([.., Pat::KwLet,
        Pat::Assign(n, e), Pat::KwIn, Pat::Blank,
        p], follow_pat)
        if follow_pat.not_blank() && p.is_expr() => {
            let top = Pat::Let(
                n.to_string(),
                Box::new(*e.clone()),
                Box::new(p.clone()),
            );
            stack.reduce_to_new(5, top)
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

fn go(stack: &Vec<Pat>, seq: Vec<Either<char, Keyword>>) -> Pat {
    let (head, tail, follow) =
        vec_get_head_tail_follow(seq);

    let follow_pat = parse_follow_pat(follow);

    let stack = stack.push_to_new(move_in(stack, head));
    println!("Move in result: {:?} follow: {:?}", stack, follow_pat);

    let reduced_stack = reduce_stack(&stack, &follow_pat);

    match (&reduced_stack[..], follow_pat) {
        ([p], FollowPat::End) => {
            let r = p.clone();
            println!("Success with: {:?}", r);
            return r;
        }
        _ => go(&reduced_stack, tail)
    }
}

pub fn parse_expr(seq: &str) -> Option<Expr> {
    println!("\nParsing seq: {:?}", seq);
    let seq = preprocess_comment(seq);
    let seq = preprocess_blank(&seq);
    let seq = preprocess_keyword(&seq);
    Option::<Expr>::from(go(&vec![Pat::Start], seq))
}

#[cfg(test)]
mod tests {
    use crate::parser::BoxExt;
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
    fn test_parse_expr_apply_part6() {
        // Apply(EnvRef, Apply(EnvRef, Unit))
        let r = Some(Expr::Apply(
            Box::new(Expr::Apply(
                Box::new(Expr::EnvRef("abc".to_string())),
                Box::new(Expr::Int(123)),
            )),
            Box::new(Expr::Apply(
                Box::new(Expr::Apply(
                    Box::new(Expr::EnvRef("add".to_string())),
                    Box::new(Expr::Int(123)),
                )),
                Box::new(Expr::Int(456)),
            )
            )));
        assert_eq!(parse_expr("abc 123 (add 123 456)"), r);
        assert_eq!(parse_expr("abc ((123)) (((add 123 456)))"), r);
        assert_eq!(parse_expr("(((abc (((123))) (((add (((123))) (((456)))))))))"), r);
    }

    #[test]
    fn test_parse_expr_apply_part7() {
        // Apply(EnvRef, Apply(EnvRef, Unit))
        let r = Some(Expr::Apply(
            Box::new(Expr::Apply(
                Box::new(Expr::EnvRef("abc".to_string())),
                Box::new(Expr::Apply(
                    Box::new(Expr::Apply(
                        Box::new(Expr::EnvRef("add".to_string())),
                        Box::new(Expr::Int(123)),
                    )),
                    Box::new(Expr::Int(456)),
                )
                ))),
            Box::new(Expr::Int(123)),
        ));
        assert_eq!(parse_expr("abc (add 123 456) 123"), r);
        assert_eq!(parse_expr("abc (((add 123 456))) ((123))"), r);
        assert_eq!(parse_expr("(((abc (((add (((123))) (((456)))))) (((123))))))"), r);
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

    #[test]
    fn test_parse_closure_part1() {
        let r = Some(Expr::Closure(
            "a".to_string(),
            Box::new(Expr::Apply(
                Box::new(Expr::Apply(
                    Box::new(Expr::EnvRef("add".to_string())),
                    Box::new(Expr::Int(123)),
                )),
                Box::new(Expr::Unit),
            ),
            )));
        let seq = "a -> add 123 ()";
        assert_eq!(parse_expr(seq), r);
        let seq = "(a -> (add (123) (())))";
        assert_eq!(parse_expr(seq), r);
        let seq = "(((a -> ((((add 123)) ((())))))))";
        assert_eq!(parse_expr(seq), r);
    }

    #[test]
    fn test_parse_closure_part2() {
        let r = Some(Expr::Closure(
            "a".to_string(),
            Box::new(Expr::Closure(
                "b".to_string(),
                Box::new(Expr::Closure(
                    "c".to_string(),
                    Box::new(Expr::Apply(
                        Box::new(Expr::Apply(
                            Box::new(Expr::EnvRef("add".to_string())),
                            Box::new(Expr::Apply(
                                Box::new(Expr::Apply(
                                    Box::new(Expr::EnvRef("add".to_string())),
                                    Box::new(Expr::EnvRef("a".to_string())),
                                )),
                                Box::new(Expr::EnvRef("b".to_string())),
                            )),
                        )),
                        Box::new(Expr::EnvRef("c".to_string())),
                    )),
                ),
                )),
            )));
        let seq = "a -> b -> c -> add (add a b) c";
        assert_eq!(parse_expr(seq), r);
        let seq = "((a -> ((b -> ((c -> ((add (((add (a) (b)))) (c)))))))))";
        assert_eq!(parse_expr(seq), r);
        let seq = "((((((a))) -> (((b -> (((c))) -> (((add))) (add a b) c))))))";
        assert_eq!(parse_expr(seq), r);
    }

    #[test]
    fn test_parse_closure_part3() {
        let r = Some(Expr::Closure(
            "aaa".to_string(),
            Box::new(Expr::Closure(
                "bbb".to_string(),
                Box::new(Expr::Closure(
                    "ccc".to_string(),
                    Box::new(Expr::Apply(
                        Box::new(Expr::Apply(
                            Box::new(Expr::EnvRef("add".to_string())),
                            Box::new(Expr::Apply(
                                Box::new(Expr::Apply(
                                    Box::new(Expr::EnvRef("add".to_string())),
                                    Box::new(Expr::EnvRef("aaa".to_string())),
                                )),
                                Box::new(Expr::Int(123)),
                            )),
                        )),
                        Box::new(Expr::EnvRef("ccc".to_string())),
                    )),
                ),
                )),
            )));
        let seq = "aaa -> bbb -> ccc -> add (add aaa 123) ccc";
        assert_eq!(parse_expr(seq), r);
        let seq = "(((aaa -> ((bbb -> (ccc -> ((((((add (add aaa 123)))) ccc)))))))))";
        assert_eq!(parse_expr(seq), r);
        let seq = "(((aaa -> (((((bbb))) -> (((ccc)) -> ((((((add (add (((aaa))) 123)))) ccc)))))))))";
        assert_eq!(parse_expr(seq), r);
    }

    #[test]
    fn test_parse_struct_part1() {
        let r = Some(Expr::Struct(vec![
            ("a".to_string(), Expr::Int(123)),
            ("ab".to_string(), Expr::EnvRef("ref".to_string())),
            ("abc".to_string(), Expr::Unit),
        ]));
        let seq = "{ a = 123, ab = ref, abc = () }";
        assert_eq!(parse_expr(seq), r);
        let seq = "(({ a = (((123))), ab = (((ref))), abc = ((())) }))";
        assert_eq!(parse_expr(seq), r);
    }

    #[test]
    fn test_parse_struct_part2() {
        let a = Expr::Struct(vec![
            ("abc".to_string(),
             Expr::Struct(vec![
                 ("efg".to_string(), Expr::Cond(
                     Box::new(Expr::Int(123)),
                     Box::new(Expr::Unit),
                     Box::new(Expr::Int(0)),
                 ))
             ])),
            ("x".to_string(), Expr::Int(1)),
        ]);
        let f = Expr::Closure(
            "x".to_string(),
            Box::new(Expr::Closure(
                "y".to_string(),
                Box::new(Expr::Apply(
                    Box::new(Expr::Apply(
                        Box::new(Expr::EnvRef("add".to_string())),
                        Box::new(Expr::EnvRef("x".to_string())),
                    )),
                    Box::new(Expr::EnvRef("y".to_string())),
                )),
            )),
        );
        let r = Some(Expr::Struct(vec![
            ("a".to_string(), a),
            ("ab".to_string(), Expr::Apply(
                Box::new(Expr::EnvRef("neg".to_string())),
                Box::new(Expr::Int(1)),
            )),
            ("f".to_string(), f),
        ]));
        let seq =
            "{ \
               a = { abc = { efg = if 123 then () else 0 }, x = 1 }, \
               ab = neg 1, \
               f = (x -> y -> add x y) \
             }";
        assert_eq!(parse_expr(seq), r);
        let seq =
            "((({ \
                  a = ((({ abc = { efg = if 123 then ((())) else 0 }, x = 1 }))), \
                  ab = (((neg))) 1, \
                  f = (x -> y -> add x y) \
            })))";
        assert_eq!(parse_expr(seq), r);
        let seq =
            "((({ \
                  (((a))) = ((({ abc = { efg = if (((123))) then ((())) else 0 }, x = (((1))) }))), \
                  (((ab))) = ((((((neg))) (((1)))))), \
                  (((f))) = (x -> (((y -> add x y)))) \
            })))";
        assert_eq!(parse_expr(seq), r);
    }

    #[test]
    fn test_parse_match_part1() {
        let r = Some(Expr::Match(
            Box::new(Expr::EnvRef("x".to_string())),
            vec![
                (Expr::Int(1),
                 Expr::Apply(
                     Box::new(Expr::EnvRef("neg".to_string())),
                     Box::new(Expr::Int(1)),
                 )),
                (Expr::Int(2),
                 Expr::Cond(
                     Box::new(Expr::EnvRef("abc".to_string())),
                     Box::new(Expr::Unit),
                     Box::new(Expr::Int(0)),
                 )),
                (Expr::Struct(vec![
                    ("a".to_string(), Expr::Int(1)),
                    ("b".to_string(), Expr::Discard),
                    ("c".to_string(), Expr::Int(3)),
                ]),
                 Expr::Int(0)),
                (Expr::Discard,
                 Expr::Unit),
            ],
        ));

        let seq =
            "match x with\
             | 1 -> neg 1\
             | 2 -> if abc then () else 0\
             | { a = 1, b = _, c = 3 } -> 0\
             | _ -> ()";
        assert_eq!(parse_expr(seq), r);
        let seq =
            "(((\
               match x with\
               | (((1))) -> (((neg 1)))\
               | (((2))) -> (((if (((abc))) then (((()))) else (((0))))))\
               | ((({ a = (((1))), b = (((_))), c = (((3))) }))) -> 0\
               | (((_))) -> (((())))\
             )))";
        assert_eq!(parse_expr(seq), r);
    }

    #[test]
    fn test_parse_match_part2() {
        let r = Some(Expr::Match(
            Box::new(Expr::EnvRef("x".to_string())),
            vec![
                (Expr::Int(1),
                 Expr::Cond(
                     Box::new(Expr::EnvRef("a".to_string())),
                     Box::new(Expr::EnvRef("b".to_string())),
                     Box::new(Expr::EnvRef("c".to_string())),
                 )),
                (Expr::EnvRef("v".to_string()),
                 Expr::Closure(
                     "a".to_string(),
                     Box::new(Expr::Closure(
                         "b".to_string(),
                         Box::new(Expr::Apply(
                             Box::new(Expr::Apply(
                                 Box::new(Expr::EnvRef("add".to_string())),
                                 Box::new(Expr::EnvRef("a".to_string())),
                             )),
                             Box::new(Expr::EnvRef("b".to_string())))
                         ))
                     ))),
                (Expr::Struct(vec![
                    ("a".to_string(), Expr::Discard),
                    ("b".to_string(),
                     Expr::Struct(vec![
                         ("foo".to_string(), Expr::Discard),
                         ("bar".to_string(), Expr::Discard),
                     ])),
                    ("c".to_string(), Expr::Int(3)),
                ]),
                 Expr::Struct(vec![
                     ("x".to_string(), Expr::Int(123)),
                     ("y".to_string(), Expr::EnvRef("c".to_string())),
                 ])),
                (Expr::Discard,
                 Expr::Match(
                     Box::new(Expr::EnvRef("y".to_string())),
                     vec![
                         (Expr::Int(1), Expr::Unit),
                         (Expr::Unit, Expr::Closure(
                             "a".to_string(),
                             Box::new(Expr::Closure(
                                 "b".to_string(),
                                 Box::new(Expr::Match(
                                     Box::new(Expr::EnvRef("z".to_string())),
                                     vec![
                                         (Expr::Discard, Expr::Int(114514)),
                                         (Expr::EnvRef("a".to_string()),
                                          Expr::Closure(
                                              "x".to_string(),
                                              Box::new(Expr::Closure(
                                                  "y".to_string(),
                                                  Box::new(Expr::Apply(
                                                      Box::new(Expr::Apply(
                                                          Box::new(Expr::EnvRef("add".to_string())),
                                                          Box::new(Expr::Unit),
                                                      )),
                                                      Box::new(Expr::EnvRef("y".to_string())),
                                                  )),
                                              )),
                                          )),
                                     ],
                                 )),
                             )),
                         )),
                         (Expr::Discard, Expr::EnvRef("baz".to_string())),
                     ],
                 )),
            ],
        ));

        let seq =
            "match x with\
             | 1 -> if a then b else c\
             | v -> a -> b -> add a b\
             | { a = _, b = { foo = _, bar = _ }, c = 3 } -> \
                 { x = 123, y = c }\
             | _ -> \
                match y with\
                | 1 -> ()\
                | () -> \
                     a -> b -> \
                       (\
                       match z with\
                       | _ -> 114514\
                       | a -> x -> y -> add () y\
                       )\
                | _ -> baz";

        assert_eq!(parse_expr(seq), r);

        let seq =
            "(((\
            match (((x))) with\
            | 1 -> if a then b else c\
            | (((v))) -> a -> b -> (((add a b)))\
            | { a = (((_))), b = { foo = (((_))), bar = (((_))) }, c = 3 } -> \
                ((({ x = (((123))), y = c })))\
            | (((_))) -> \
               (((\
               match y with\
               | 1 -> ()\
               | () -> \
                    (((\
                    a -> b -> \
                      (((\
                      match (((z))) with\
                      | (((_))) -> 114514\
                      | (((a))) -> \
                        (((\
                        (((x))) -> (((y))) -> (((add () y)))\
                        )))\
                      )))\
                    )))\
               | _ -> baz\
               )))\
             )))";

        assert_eq!(parse_expr(seq), r);
    }

    #[test]
    fn test_parse_let_part1() {
        let r = Some(Expr::Let(
            "a".to_string(),
            Box::new(Expr::Int(123)),
            Box::new(Expr::Apply(
                Box::new(Expr::Apply(
                    Box::new(Expr::EnvRef("add".to_string())),
                    Box::new(Expr::EnvRef("a".to_string())),
                )),
                Box::new(Expr::Int(456)),
            )),
        ));

        let seq = "let a = 123 in add a 456";
        assert_eq!(parse_expr(seq), r);
        let seq = "(((let (((a))) = (((123))) in (((add a (((456)))))))))";
        assert_eq!(parse_expr(seq), r);
    }

    #[test]
    fn test_parse_let_part2() {
        let r = Some(Expr::Let(
            "a".to_string(),
            Box::new(Expr::Int(123)),
            Box::new(Expr::Let(
                "b".to_string(),
                Box::new(Expr::Apply(
                    Box::new(Expr::Apply(
                        Box::new(Expr::EnvRef("add".to_string())),
                        Box::new(Expr::EnvRef("c".to_string())),
                    )),
                    Box::new(Expr::EnvRef("d".to_string())),
                )),
                Box::new(Expr::Apply(
                    Box::new(Expr::Apply(
                        Box::new(Expr::EnvRef("add".to_string())),
                        Box::new(Expr::Unit),
                    )),
                    Box::new(Expr::Int(456)),
                )),
            )),
        ));

        let seq = "let a = 123, b = add c d in add () 456";
        assert_eq!(parse_expr(seq), r);
    }

    #[test]
    fn test_parse_let_part3() {
        let r = Expr::Let(
            "a".to_string(),
            Expr::Int(123).boxed(),
            Expr::Let(
                "b".to_string(),
                Expr::Let("x".to_string(),
                          Expr::Closure(
                              "i".to_string(),
                              Expr::Closure(
                                  "j".to_string(),
                                  Expr::EnvRef("k".to_string()).boxed(),
                              ).boxed(),
                          ).boxed(),
                          Expr::Let(
                              "y".to_string(),
                              Expr::EnvRef("a".to_string()).boxed(),
                              Expr::Let(
                                  "z".to_string(),
                                  Expr::Unit.boxed(),
                                  Expr::EnvRef("a".to_string()).boxed(),
                              ).boxed(),
                          ).boxed(),
                ).boxed(),
                Expr::Let(
                    "d".to_string(),
                    Expr::Apply(
                        Expr::EnvRef("neg".to_string()).boxed(),
                        Expr::Int(1).boxed(),
                    ).boxed(),
                    Expr::Let(
                        "e".to_string(),
                        Expr::Int(6).boxed(),
                        Expr::Let(
                            "k".to_string(),
                            Expr::Unit.boxed(),
                            Expr::Let(
                                "m".to_string(),
                                Expr::Unit.boxed(),
                                Expr::Let(
                                    "n".to_string(),
                                    Expr::Int(4).boxed(),
                                    Expr::Apply(
                                        Expr::Apply(
                                            Expr::EnvRef("add".to_string()).boxed(),
                                            Expr::Unit.boxed(),
                                        ).boxed(),
                                        Expr::Int(456).boxed(),
                                    ).boxed(),
                                ).boxed(),
                            ).boxed(),
                        ).boxed(),
                    ).boxed(),
                ).boxed(),
            ).boxed(),
        );
        let r = Some(r);

        let seq =
            "let a = 123, \
                 b = \
                 let x = i -> j -> k, \
                     y = a \
                 in let z = () in a, \
                 d = neg 1 \
             in \
             let e = 6, k = () in \
             let m = (), n = 4 in \
             add () 456";
        assert_eq!(parse_expr(seq), r);
        let seq =
            "let a = (((123))), \
                 b = \
                 (((\
                     let x = ((((((i))) -> ((((((j))) -> (((k))))))))), \
                         y = (((a))) \
                     in (((\
                        let (((z))) = (((()))) in (((a)))\
                        )))\
                 ))), \
                 (((d))) = \
                     (((\
                         (((neg))) (((1)))\
                     ))) \
             in \
             (((\
             let (((e))) = (((6))), (((k))) = (((()))) in \
                 (((\
                 let (((m))) = (((()))), (((n))) = (((4))) in \
                     (((\
                     add () (((456)))\
                     )))\
                 )))\
             )))";
        assert_eq!(parse_expr(seq), r);
    }
}
