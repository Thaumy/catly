use crate::parser::expr::Expr;
use crate::parser::infra::{BoxExt, MaybeExpr, MaybeType, VecExt};
use crate::parser::keyword::Keyword;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Pat {
    Start,
    End,
    Err,

    Mark(char),

    Unit,//Expr::Unit

    Digit(char),
    DigitSeq(String),
    Int(i64),//Expr::Int

    Alphanum(char),
    AlphanumSeq(String),
    LetName(String),//Expr::EnvRef

    Blank,
    Apply(Box<Pat>, Box<Pat>),//Expr::Apply

    KwIf,
    KwThen,
    KwElse,
    Cond(MaybeType, Box<Pat>, Box<Pat>, Box<Pat>),//Expr::Cond

    Arrow,
    ClosurePara(String),
    Closure(MaybeType, String, MaybeType, Box<Pat>),//Expr::Closure

    Assign(String, Box<Pat>),
    AssignSeq(Vec<(String, Pat)>),
    Struct(Vec<(String, Pat)>),//Expr::Struct

    KwMatch,
    KwWith,
    Discard,
    MatchHead(Box<Pat>),
    CaseHead(Box<Pat>),
    Case(Box<Pat>, Box<Pat>),
    CaseSeq(Vec<(Pat, Pat)>),
    Match(MaybeType, Box<Pat>, Vec<(Pat, Pat)>),//Expr::Match

    KwLet,
    KwIn,
    Let(MaybeType, String, MaybeType, Box<Pat>, Box<Pat>),
}

impl Pat {
    pub(crate) fn is_expr(&self) -> bool {
        match self {
            Pat::Unit |
            Pat::Int(_) |
            Pat::LetName(_) |
            Pat::Apply(_, _) |
            Pat::Cond(_, _, _, _) |
            Pat::Closure(_, _, _, _) |
            Pat::Struct(_) |
            Pat::Discard |
            Pat::Match(_, _, _) |
            Pat::Let(_, _, _, _, _)
            => true,
            _ => false,
        }
    }
}

impl From<Keyword> for Pat {
    fn from(kw: Keyword) -> Self {
        match kw {
            Keyword::Type => todo!(),
            Keyword::Def => todo!(),
            Keyword::Let => Pat::KwLet,
            Keyword::In => Pat::KwIn,
            Keyword::If => Pat::KwIf,
            Keyword::Then => Pat::KwThen,
            Keyword::Else => Pat::KwElse,
            Keyword::Match => Pat::KwMatch,
            Keyword::With => Pat::KwWith,
        }
    }
}

impl From<Pat> for MaybeExpr {
    fn from(pat: Pat) -> Self {
        let r = match pat {
            Pat::Discard => Expr::Discard,
            Pat::Unit => Expr::Unit(None),
            Pat::Int(i) => Expr::Int(None, i),
            Pat::LetName(n) => Expr::EnvRef(n),
            Pat::Apply(l, r) =>
                match (Self::from(*l), Self::from(*r)) {
                    (Some(l), Some(r)) =>
                        Expr::Apply(
                            None,
                            l.boxed(),
                            r.boxed(),
                        ),
                    _ => return None
                }
            Pat::Cond(_, a, b, c) =>
                match (Self::from(*a), Self::from(*b), Self::from(*c)) {
                    (Some(a), Some(b), Some(c)) =>
                        Expr::Cond(
                            None,
                            a.boxed(),
                            b.boxed(),
                            c.boxed(),
                        ),
                    _ => return None
                }
            Pat::Closure(_, para, _, e) =>
                match Self::from(*e) {
                    Some(e) => Expr::Closure(
                        None,
                        para,
                        None,
                        e.boxed(),
                    ),
                    _ => return None
                }
            Pat::Struct(vec) => {
                type Assign = (String, MaybeType, Expr);
                type F = fn(Option<Vec<Assign>>, &(String, Pat)) -> Option<Vec<Assign>>;
                let f: F = |acc, (n, p)|
                    match (acc, Self::from(p.clone())) {
                        (Some(vec), Some(e)) =>
                            Some(vec.push_to_new((n.to_string(), None, e))),
                        _ => None,
                    };
                let vec = vec.iter().fold(Some(vec![]), f);

                match vec {
                    Some(vec) => Expr::Struct(None, vec),
                    _ => return None,
                }
            }
            Pat::Match(_, p, vec) => {
                type Case = (Expr, Expr);
                type F = fn(Option<Vec<Case>>, &(Pat, Pat)) -> Option<Vec<Case>>;
                let f: F = |acc, (case_p, then_p)|
                    match (acc, Self::from(case_p.clone()), Self::from(then_p.clone())) {
                        (Some(vec), Some(case_e), Some(then_e)) =>
                            Some(vec.push_to_new((case_e, then_e))),
                        _ => None,
                    };
                let vec = vec.iter().fold(Some(vec![]), f);

                match (Self::from(*p), vec) {
                    (Some(p), Some(vec)) =>
                        Expr::Match(
                            None,
                            p.boxed(),
                            vec,
                        ),
                    _ => Expr::Unit(None)
                }
            }
            Pat::Let(_, n, _, n_e, e) =>
                match (Self::from(*n_e), Self::from(*e)) {
                    (Some(n_e), Some(e)) =>
                        Expr::Let(
                            None,
                            n,
                            None,
                            n_e.boxed(),
                            e.boxed(),
                        ),
                    _ => return None
                }

            _ => return None,
        };
        Some(r)
    }
}
