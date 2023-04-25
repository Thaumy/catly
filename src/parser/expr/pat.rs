use std::collections::BTreeSet;

use crate::infra::btree_set::Ext;
use crate::infra::iter::IntoIteratorExt;
use crate::infra::option::AnyExt;
use crate::infra::r#box::Ext as BoxAnyExt;
use crate::infra::vec::Ext as VecAnyExt;
use crate::parser::expr::r#type::{Expr, MaybeExpr};
use crate::parser::keyword::Keyword;
use crate::parser::r#type::r#type::{MaybeType, Type};

pub type OptBoxPat = Option<Box<Pat>>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pat {
    Start,
    End,
    Err,

    Mark(char),
    Kw(Keyword),

    /* expression patterns */
    // Expr::Unit
    Unit(OptBoxPat),
    // Expr::Int
    Int(OptBoxPat, i64),

    LetName(OptBoxPat, String), // Expr::EnvRef

    Apply(OptBoxPat, Box<Pat>, Box<Pat>), // Expr::Apply

    // if then else
    Cond(OptBoxPat, Box<Pat>, Box<Pat>, Box<Pat>), // Expr::Cond

    Arrow,
    ClosureInput(Option<String>, OptBoxPat),
    Closure(OptBoxPat, Option<String>, OptBoxPat, Box<Pat>), // Expr::Closure

    Assign(String, OptBoxPat, Box<Pat>),
    AssignSeq(Vec<(String, OptBoxPat, Pat)>),
    Struct(OptBoxPat, Vec<(String, OptBoxPat, Pat)>), // Expr::Struct

    // match with
    Discard(OptBoxPat),
    MatchHead(Box<Pat>),
    CaseHead(Box<Pat>),
    Case(Box<Pat>, Box<Pat>),
    CaseSeq(Vec<(Pat, Pat)>),
    Match(OptBoxPat, Box<Pat>, Vec<(Pat, Pat)>), // Expr::Match

    // let in
    Let(OptBoxPat, String, OptBoxPat, Box<Pat>, Box<Pat>),

    /* type annotation patterns */
    TypedExprHead(Box<Pat>),

    TypeName(String), // Type::NamelyType

    TypeApply(Box<Pat>, Box<Pat>), // Type::TypeApply

    ClosureTypeHead(Box<Pat>),
    ClosureType(Box<Pat>, Box<Pat>), // Type::ClosureType

    SumType(BTreeSet<Pat>), // Type::SumType

    TypedLetNameSeq(Vec<(String, Pat)>),
    ProdType(Vec<(String, Pat)>) // Type::ProdType
}

impl Pat {
    pub fn is_expr(&self) -> bool {
        match self {
            Pat::Unit(..) |
            Pat::Int(..) |
            Pat::LetName(..) |
            Pat::Apply(..) |
            Pat::Cond(..) |
            Pat::Closure(..) |
            Pat::Struct(..) |
            Pat::Discard(..) |
            Pat::Match(..) |
            Pat::Let(..) => true,
            _ => false
        }
    }
    pub fn with_type(self, r#type: Pat) -> Option<Self> {
        let r = match self {
            Pat::Discard(_) => Pat::Discard(r#type.boxed().some()),
            Pat::Unit(_) => Pat::Unit(r#type.boxed().some()),
            Pat::Int(_, i) => Pat::Int(r#type.boxed().some(), i),
            Pat::LetName(_, n) =>
                Pat::LetName(r#type.boxed().some(), n),
            Pat::Apply(_, lhs, rhs) =>
                Pat::Apply(r#type.boxed().some(), lhs, rhs),
            Pat::Cond(_, e, t, f) =>
                Pat::Cond(r#type.boxed().some(), e, t, f),
            Pat::Closure(_, i_n, i_t, o) =>
                Pat::Closure(r#type.boxed().some(), i_n, i_t, o),
            Pat::Struct(_, vec) =>
                Pat::Struct(r#type.boxed().some(), vec),
            Pat::Match(_, e, vec) =>
                Pat::Match(r#type.boxed().some(), e, vec),
            Pat::Let(_, a_n, a_t, a_e, e) =>
                Pat::Let(r#type.boxed().some(), a_n, a_t, a_e, e),
            _ => return None
        };
        Some(r)
    }
    pub fn is_type(&self) -> bool {
        match self {
            Pat::TypeName(..) |
            Pat::TypeApply(..) |
            Pat::ClosureType(..) |
            Pat::SumType(..) |
            Pat::ProdType(..) => true,
            _ => false
        }
    }
}

impl From<Keyword> for Pat {
    fn from(kw: Keyword) -> Self { Pat::Kw(kw) }
}

pub trait OptBoxPatExt {
    fn map_into(self) -> Option<Type>;
}

impl OptBoxPatExt for Option<Box<Pat>> {
    fn map_into(self) -> Option<Type> {
        self.map(|x| <Pat as Into<MaybeType>>::into(*x))
            .flatten()
    }
}

impl From<Pat> for MaybeExpr {
    fn from(pat: Pat) -> Self {
        let r = match pat {
            Pat::Discard(t) => Expr::Discard(t.map_into()),
            Pat::Unit(t) => Expr::Unit(t.map_into()),
            Pat::Int(t, i) => Expr::Int(t.map_into(), i),
            Pat::LetName(t, n) => Expr::EnvRef(t.map_into(), n),
            Pat::Apply(t, l, r) => {
                match (Self::from(*l), Self::from(*r)) {
                    (Some(l), Some(r)) => Expr::Apply(
                        t.map_into(),
                        l.boxed(),
                        r.boxed()
                    ),
                    _ => return None
                }
            }
            Pat::Cond(t, a, b, c) => {
                match (Self::from(*a), Self::from(*b), Self::from(*c))
                {
                    (Some(a), Some(b), Some(c)) => Expr::Cond(
                        t.map_into(),
                        a.boxed(),
                        b.boxed(),
                        c.boxed()
                    ),
                    _ => return None
                }
            }
            Pat::Closure(t, i_n, i_t, o) => match Self::from(*o) {
                Some(o) => Expr::Closure(
                    t.map_into(),
                    i_n,
                    i_t.map_into(),
                    o.boxed()
                ),
                _ => return None
            },
            Pat::Struct(t, vec) => {
                let vec = vec.maybe_fold(vec![], |acc, (n, t, p)| {
                    let it =
                        (p.clone().into(): MaybeExpr).map(|e| {
                            (n.to_string(), t.clone().map_into(), e)
                        })?;
                    acc.chain_push(it).some()
                });

                match vec {
                    Some(vec) => Expr::Struct(t.map_into(), vec),
                    _ => return None
                }
            }
            Pat::Match(t, p, vec) => {
                let vec = vec.maybe_fold(
                    vec![],
                    |acc, (case_p, then_p)| {
                        let case_e =
                            (case_p.clone().into(): MaybeExpr)?;
                        let then_e =
                            (then_p.clone().into(): MaybeExpr)?;
                        acc.chain_push((case_e, then_e))
                            .some()
                    }
                );

                match (Self::from(*p), vec) {
                    (Some(p), Some(vec)) =>
                        Expr::Match(t.map_into(), p.boxed(), vec),
                    _ => return None
                }
            }
            Pat::Let(t, a_n, a_t, a_e, e) => {
                match (Self::from(*a_e), Self::from(*e)) {
                    (Some(a_e), Some(e)) => Expr::Let(
                        t.map_into(),
                        a_n,
                        a_t.map_into(),
                        a_e.boxed(),
                        e.boxed()
                    ),
                    _ => return None
                }
            }

            _ => return None
        };
        Some(r)
    }
}

impl From<Pat> for MaybeType {
    fn from(pat: Pat) -> Self {
        match pat {
            Pat::TypeName(n) => Type::NamelyType(n),

            Pat::ClosureType(i, o) => Type::ClosureType(
                Self::from(*i)?.boxed(),
                Self::from(*o)?.boxed()
            ),

            Pat::SumType(s_s) => s_s
                .maybe_fold(BTreeSet::new(), |acc, t| {
                    let t: MaybeType = t.clone().into();
                    acc.chain_insert(t?).some()
                })
                .map(|set| Type::SumType(set))?,

            Pat::ProdType(p_v) => p_v
                .maybe_fold(vec![], |acc, (n, p)| {
                    let n = n.to_string();
                    let t: MaybeType = p.clone().into();
                    acc.chain_push((n, t?)).some()
                })
                .map(|vec| Type::ProdType(vec))?,
            _ => return None
        }
        .some()
    }
}
