use std::collections::BTreeSet;
use std::ops::Deref;
use std::rc::Rc;

use crate::infra::btree_set::BtreeSetExt;
use crate::infra::option::WrapOption;
use crate::infra::rc::RcAnyExt;
use crate::infra::vec::VecExt;
use crate::parser::expr::r#type::{Expr, OptExpr};
use crate::parser::keyword::Keyword;
use crate::parser::r#type::r#type::{OptType, Type};

pub type OptRcPat = Option<Rc<Pat>>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pat {
    Start,
    End,
    Err,

    Mark(char),
    Kw(Keyword),

    /* expression patterns */
    // Expr::Unit
    Unit(OptRcPat),
    // Expr::Int
    Int(OptRcPat, i64),

    LetName(OptRcPat, String), // Expr::EnvRef

    Apply(OptRcPat, Rc<Pat>, Rc<Pat>), // Expr::Apply

    // if then else
    Cond(OptRcPat, Rc<Pat>, Rc<Pat>, Rc<Pat>), // Expr::Cond

    Arrow,
    ClosureInput(Option<String>, OptRcPat),
    Closure(OptRcPat, Option<String>, OptRcPat, Rc<Pat>), // Expr::Closure

    Assign(bool, String, OptRcPat, Rc<Pat>),
    AssignSeq(Vec<(bool, String, OptRcPat, Pat)>),
    Struct(OptRcPat, Vec<(String, OptRcPat, Pat)>), // Expr::Struct

    // match with
    Discard(OptRcPat),
    MatchHead(Rc<Pat>),
    CaseHead(Rc<Pat>),
    Case(Rc<Pat>, Rc<Pat>),
    CaseSeq(Vec<(Pat, Pat)>),
    Match(OptRcPat, Rc<Pat>, Vec<(Pat, Pat)>), // Expr::Match

    // let in
    Let(OptRcPat, bool, String, OptRcPat, Rc<Pat>, Rc<Pat>),

    /* type annotation patterns */
    TypedExprHead(Rc<Pat>),

    TypeName(String), // Type::NamelyType

    TypeApply(Rc<Pat>, Rc<Pat>), // Type::TypeApply

    ClosureTypeHead(Rc<Pat>),
    ClosureType(Rc<Pat>, Rc<Pat>), // Type::ClosureType

    SumType(BTreeSet<Pat>), // Type::SumType

    TypedLetNameSeq(Vec<(String, Pat)>),
    ProdType(Vec<(String, Pat)>) // Type::ProdType
}

impl Pat {
    pub fn is_expr(&self) -> bool {
        matches!(
            self,
            Pat::Unit(..) |
                Pat::Int(..) |
                Pat::LetName(..) |
                Pat::Apply(..) |
                Pat::Cond(..) |
                Pat::Closure(..) |
                Pat::Struct(..) |
                Pat::Discard(..) |
                Pat::Match(..) |
                Pat::Let(..)
        )
    }
    pub fn with_type(self, r#type: Pat) -> Option<Self> {
        match self {
            Pat::Discard(_) => Pat::Discard(r#type.rc().wrap_some()),
            Pat::Unit(_) => Pat::Unit(r#type.rc().wrap_some()),
            Pat::Int(_, i) => Pat::Int(r#type.rc().wrap_some(), i),
            Pat::LetName(_, n) =>
                Pat::LetName(r#type.rc().wrap_some(), n),
            Pat::Apply(_, lhs, rhs) =>
                Pat::Apply(r#type.rc().wrap_some(), lhs, rhs),
            Pat::Cond(_, e, t, f) =>
                Pat::Cond(r#type.rc().wrap_some(), e, t, f),
            Pat::Closure(_, i_n, i_t, o) =>
                Pat::Closure(r#type.rc().wrap_some(), i_n, i_t, o),
            Pat::Struct(_, vec) =>
                Pat::Struct(r#type.rc().wrap_some(), vec),
            Pat::Match(_, e, vec) =>
                Pat::Match(r#type.rc().wrap_some(), e, vec),
            Pat::Let(_, r_a, a_n, a_t, a_e, e) => Pat::Let(
                r#type.rc().wrap_some(),
                r_a,
                a_n,
                a_t,
                a_e,
                e
            ),
            _ => return None
        }
        .wrap_some()
    }
    pub fn is_type(&self) -> bool {
        matches!(
            self,
            Pat::TypeName(..) |
                Pat::TypeApply(..) |
                Pat::ClosureType(..) |
                Pat::SumType(..) |
                Pat::ProdType(..)
        )
    }
}

impl From<Keyword> for Pat {
    fn from(kw: Keyword) -> Self { Pat::Kw(kw) }
}

pub trait OptRcPatExt {
    fn map_into(self) -> Option<Type>;
}

impl OptRcPatExt for Option<Rc<Pat>> {
    fn map_into(self) -> Option<Type> {
        self.and_then(|x| {
            <Pat as Into<OptType>>::into(x.deref().clone())
        })
    }
}

impl From<Pat> for OptExpr {
    fn from(pat: Pat) -> Self {
        match pat {
            Pat::Discard(t) => Expr::Discard(t.map_into()),
            Pat::Unit(t) => Expr::Unit(t.map_into()),
            Pat::Int(t, i) => Expr::Int(t.map_into(), i),
            Pat::LetName(t, n) => Expr::EnvRef(t.map_into(), n),
            Pat::Apply(t, l, r) => {
                match (
                    Self::from(l.deref().clone()),
                    Self::from(r.deref().clone())
                ) {
                    (Some(l), Some(r)) =>
                        Expr::Apply(t.map_into(), l.rc(), r.rc()),
                    _ => return None
                }
            }
            Pat::Cond(t, a, b, c) => {
                match (
                    Self::from(a.deref().clone()),
                    Self::from(b.deref().clone()),
                    Self::from(c.deref().clone())
                ) {
                    (Some(a), Some(b), Some(c)) => Expr::Cond(
                        t.map_into(),
                        a.rc(),
                        b.rc(),
                        c.rc()
                    ),
                    _ => return None
                }
            }
            Pat::Closure(t, i_n, i_t, o) =>
                match Self::from(o.deref().clone()) {
                    Some(o) => Expr::Closure(
                        t.map_into(),
                        i_n,
                        i_t.map_into(),
                        o.rc()
                    ),
                    _ => return None
                },
            Pat::Struct(t, vec) => {
                let vec = vec.into_iter().try_fold(
                    vec![],
                    |acc, (n, t, p)| {
                        let it = OptExpr::from(p)
                            .map(|e| (n, t.map_into(), e))?;
                        acc.chain_push(it).wrap_some()
                    }
                );

                match vec {
                    Some(vec) => Expr::Struct(t.map_into(), vec),
                    _ => return None
                }
            }
            Pat::Match(t, p, vec) => {
                let vec = vec.into_iter().try_fold(
                    vec![],
                    |acc, (case_p, then_p)| {
                        let case_e = OptExpr::from(case_p)?;
                        let then_e = OptExpr::from(then_p)?;
                        acc.chain_push((case_e, then_e))
                            .wrap_some()
                    }
                );

                match (Self::from(p.deref().clone()), vec) {
                    (Some(p), Some(vec)) =>
                        Expr::Match(t.map_into(), p.rc(), vec),
                    _ => return None
                }
            }
            Pat::Let(t, r_a, a_n, a_t, a_e, e) => {
                match (
                    Self::from(a_e.deref().clone()),
                    Self::from(e.deref().clone())
                ) {
                    (Some(a_e), Some(e)) => Expr::Let(
                        t.map_into(),
                        r_a,
                        a_n,
                        a_t.map_into(),
                        a_e.rc(),
                        e.rc()
                    ),
                    _ => return None
                }
            }

            _ => return None
        }
        .wrap_some()
    }
}

impl From<Pat> for OptType {
    fn from(pat: Pat) -> Self {
        match pat {
            Pat::TypeName(n) => Type::NamelyType(n),

            Pat::ClosureType(i, o) => Type::ClosureType(
                Self::from(i.deref().clone())?.rc(),
                Self::from(o.deref().clone())?.rc()
            ),

            Pat::SumType(s_s) => s_s
                .into_iter()
                .try_fold(BTreeSet::new(), |acc, t| {
                    let t: OptType = t.into();
                    acc.chain_insert(t?)
                        .wrap_some()
                })
                .map(Type::SumType)?,

            Pat::ProdType(p_v) => p_v
                .into_iter()
                .try_fold(vec![], |acc, (n, p)| {
                    let t: OptType = p.into();
                    acc.chain_push((n, t?))
                        .wrap_some()
                })
                .map(Type::ProdType)?,
            _ => return None
        }
        .wrap_some()
    }
}
