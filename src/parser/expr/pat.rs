use std::collections::BTreeSet;
use crate::maybe_fold;
use crate::parser::expr::Expr;
use crate::parser::infra::alias::{MaybeExpr, MaybeType};
use crate::parser::infra::r#box::Ext;
use crate::parser::keyword::Keyword;
use crate::parser::r#type::Type;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum Pat {
    Start,
    End,
    Err,

    Mark(char),
    Kw(Keyword),

    /* expression patterns */

    // Expr::Unit
    Unit(MaybeType),
    // Expr::Int
    Int(MaybeType, i64),

    LetName(MaybeType, String),// Expr::EnvRef

    Apply(MaybeType, Box<Pat>, Box<Pat>),// Expr::Apply

    // if then else
    Cond(MaybeType, Box<Pat>, Box<Pat>, Box<Pat>),// Expr::Cond

    Arrow,
    ClosurePara(String, MaybeType),
    Closure(MaybeType, String, MaybeType, Box<Pat>),// Expr::Closure

    Assign(String, MaybeType, Box<Pat>),
    AssignSeq(Vec<(String, MaybeType, Pat)>),
    Struct(MaybeType, Vec<(String, MaybeType, Pat)>),// Expr::Struct

    // match with
    Discard(MaybeType),
    MatchHead(Box<Pat>),
    CaseHead(Box<Pat>),
    Case(Box<Pat>, Box<Pat>),
    CaseSeq(Vec<(Pat, Pat)>),
    Match(MaybeType, Box<Pat>, Vec<(Pat, Pat)>),// Expr::Match

    // let in
    Let(MaybeType, String, MaybeType, Box<Pat>, Box<Pat>),

    /* type annotation patterns */

    TypedExprHead(Box<Pat>),

    TypeName(String),// Type::TypeEnvRef

    TypeApply(Box<Pat>, Box<Pat>),// Type::TypeApply

    ClosureTypeHead(Box<Pat>),
    ClosureType(Box<Pat>, Box<Pat>),// Type::ClosureType

    SumType(BTreeSet<Pat>),// Type::SumType

    LetNameWithType(String, Box<Pat>),
    LetNameWithTypeSeq(Vec<(String, Pat)>),
    ProductType(Vec<(String, Pat)>),// Type::ProductType
}

impl Pat {
    pub fn is_expr(&self) -> bool {
        match self {
            Pat::Unit(_) |
            Pat::Int(_, _) |
            Pat::LetName(_, _) |
            Pat::Apply(_, _, _) |
            Pat::Cond(_, _, _, _) |
            Pat::Closure(_, _, _, _) |
            Pat::Struct(_, _) |
            Pat::Discard(_) |
            Pat::Match(_, _, _) |
            Pat::Let(_, _, _, _, _)
            => true,
            _ => false,
        }
    }
    pub fn with_type(self, r#type: Type) -> Option<Self> {
        let r = match self {
            Pat::Unit(_) => Pat::Unit(
                Some(r#type)
            ),
            Pat::Int(_, i) => Pat::Int(
                Some(r#type),
                i,
            ),
            Pat::LetName(_, n) => Pat::LetName(
                Some(r#type),
                n,
            ),
            Pat::Apply(_, lhs, rhs) => Pat::Apply(
                Some(r#type),
                lhs,
                rhs,
            ),
            Pat::Cond(_, e, t, f) => Pat::Cond(
                Some(r#type),
                e,
                t,
                f,
            ),
            Pat::Closure(_, para_n, para_t, e) => Pat::Closure(
                Some(r#type),
                para_n,
                para_t,
                e,
            ),
            Pat::Struct(_, vec) => Pat::Struct(
                Some(r#type),
                vec,
            ),
            Pat::Discard(_) => Pat::Discard(
                Some(r#type)
            ),
            Pat::Match(_, e, vec) => Pat::Match(
                Some(r#type),
                e,
                vec,
            ),
            Pat::Let(_, a_n, a_t, a_e, e) => Pat::Let(
                Some(r#type),
                a_n,
                a_t,
                a_e,
                e,
            ),
            _ => return None,
        };
        Some(r)
    }
    pub fn is_type(&self) -> bool {
        match self {
            Pat::TypeName(_) |
            Pat::TypeApply(_, _) |
            Pat::ClosureType(_, _) |
            Pat::SumType(_) |
            Pat::ProductType(_)
            => true,
            _ => false,
        }
    }
}

impl From<Keyword> for Pat {
    fn from(kw: Keyword) -> Self {
        Pat::Kw(kw)
    }
}

impl From<Pat> for MaybeExpr {
    fn from(pat: Pat) -> Self {
        let r = match pat {
            Pat::Discard(t) => Expr::Discard(t),
            Pat::Unit(t) => Expr::Unit(t),
            Pat::Int(t, i) => Expr::Int(t, i),
            Pat::LetName(_, n) => Expr::EnvRef(n),// Discard the type
            Pat::Apply(t, l, r) =>
                match (Self::from(*l), Self::from(*r)) {
                    (Some(l), Some(r)) =>
                        Expr::Apply(
                            t,
                            l.boxed(),
                            r.boxed(),
                        ),
                    _ => return None
                }
            Pat::Cond(t, a, b, c) =>
                match (Self::from(*a), Self::from(*b), Self::from(*c)) {
                    (Some(a), Some(b), Some(c)) =>
                        Expr::Cond(
                            t,
                            a.boxed(),
                            b.boxed(),
                            c.boxed(),
                        ),
                    _ => return None
                }
            Pat::Closure(_, para, _, e) =>
                match Self::from(*e) {
                    Some(e) =>
                        Expr::Closure(
                            None,
                            para,
                            None,
                            e.boxed(),
                        ),
                    _ => return None
                }
            Pat::Struct(t, vec) => {
                let f = |(n, _, p): &(String, _, Pat)|
                    (p.clone().into(): MaybeExpr).map(|e| (n.to_string(), None, e));

                let vec = maybe_fold!(
                    vec.iter(),
                    vec![],
                    push,
                    f
                );

                match vec {
                    Some(vec) =>
                        Expr::Struct(
                            t,
                            vec,
                        ),
                    _ => return None,
                }
            }
            Pat::Match(t, p, vec) => {
                let f = |(case_p, then_p): &(Pat, Pat)| {
                    let case_e = (case_p.clone().into(): MaybeExpr)?;
                    let then_e = (then_p.clone().into(): MaybeExpr)?;
                    Some((case_e, then_e))
                };

                let vec = maybe_fold!(
                    vec.iter(),
                    vec![],
                    push,
                    f
                );

                match (Self::from(*p), vec) {
                    (Some(p), Some(vec)) =>
                        Expr::Match(
                            t,
                            p.boxed(),
                            vec,
                        ),
                    _ => Expr::Unit(None)
                }
            }
            Pat::Let(t, n, _, n_e, e) =>
                match (Self::from(*n_e), Self::from(*e)) {
                    (Some(n_e), Some(e)) =>
                        Expr::Let(
                            t,
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

impl From<Pat> for MaybeType {
    fn from(pat: Pat) -> Self {
        let r = match pat {
            Pat::TypeName(n) => Type::TypeEnvRef(n),

            Pat::ClosureType(para, t) =>
                match (Self::from(*para), Self::from(*t)) {
                    (Some(para), Some(t)) => Type::ClosureType(
                        para.boxed(),
                        t.boxed(),
                    ),
                    _ => return None
                },
            Pat::SumType(ts) => {
                let set = maybe_fold!(
                    ts.iter(),
                    BTreeSet::new(),
                    insert,
                    |t: &Pat| t.clone().into()
                );

                match set {
                    Some(set) => Type::SumType(set),
                    _ => return None,
                }
            }
            Pat::ProductType(vec) => {
                let f = |(n, p): &(String, Pat)|
                    (p.clone().into(): MaybeType).map(|e| (n.to_string(), e));

                let vec = maybe_fold!(
                    vec.iter(),
                    vec![],
                    push,
                    f
                );

                match vec {
                    Some(vec) => Type::ProductType(vec),
                    _ => return None,
                }
            }
            _ => return None
        };
        Some(r)
    }
}
