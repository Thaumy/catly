use std::vec;

use crate::infra::alias::{MaybeExpr, MaybeType};
use crate::infra::option::AnyExt;
use crate::parser::expr::pat::Pat;
use crate::parser::expr::r#fn::go;
use crate::parser::r#type::Type;

mod r#fn;
mod pat;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Unit(MaybeType),
    Int(MaybeType, u64),
    EnvRef(MaybeType, String),
    Apply(MaybeType, Box<Expr>, Box<Expr>),
    Cond(MaybeType, Box<Expr>, Box<Expr>, Box<Expr>),
    Closure(MaybeType, Option<String>, MaybeType, Box<Expr>),
    Struct(MaybeType, Vec<(String, MaybeType, Expr)>),
    Discard(MaybeType),
    Match(MaybeType, Box<Expr>, Vec<(Expr, Expr)>),
    Let(MaybeType, String, MaybeType, Box<Expr>, Box<Expr>)
}

type In = crate::parser::preprocess::Out;

pub fn parse_expr(seq: Vec<In>) -> MaybeExpr {
    println!("\nParsing Expr seq: {:?}", seq);
    go(vec![Pat::Start], seq).into()
}

impl Expr {
    pub fn with_fallback_type(self, r#type: &Type) -> Expr {
        match self {
            Expr::Unit(None) => Expr::Unit(r#type.clone().some()),
            Expr::Int(None, i) => Expr::Int(r#type.clone().some(), i),
            Expr::EnvRef(None, n) =>
                Expr::EnvRef(r#type.clone().some(), n),
            Expr::Apply(None, lhs, rhs) =>
                Expr::Apply(r#type.clone().some(), lhs, rhs),
            Expr::Cond(None, e, t, f) =>
                Expr::Cond(r#type.clone().some(), e, t, f),
            Expr::Closure(None, i_n, i_t, o) =>
                Expr::Closure(r#type.clone().some(), i_n, i_t, o),
            Expr::Struct(None, vec) =>
                Expr::Struct(r#type.clone().some(), vec),
            Expr::Discard(None) =>
                Expr::Discard(r#type.clone().some()),
            Expr::Match(None, e, vec) =>
                Expr::Match(r#type.clone().some(), e, vec),
            Expr::Let(None, a_n, a_t, a_e, e) =>
                Expr::Let(r#type.clone().some(), a_n, a_t, a_e, e),
            _ => self
        }
    }

    pub fn try_with_fallback_type(
        self,
        r#type: &Option<Type>
    ) -> Expr {
        match r#type {
            Some(t) => self.with_fallback_type(t),
            None => self
        }
    }

    pub fn is_no_type_annotation(&self) -> bool {
        match self {
            Expr::Unit(None) |
            Expr::Int(None, _) |
            Expr::EnvRef(None, _) |
            Expr::Apply(None, _, _) |
            Expr::Cond(None, _, _, _) |
            Expr::Closure(None, _, _, _) |
            Expr::Struct(None, _) |
            Expr::Discard(None) |
            Expr::Match(None, _, _) |
            Expr::Let(None, _, _, _, _) => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod test;
