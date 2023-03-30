use std::vec;

use crate::parser::expr::pat::Pat;
use crate::parser::expr::r#fn::go;
use crate::parser::infra::alias::{MaybeExpr, MaybeType};

mod pat;
mod r#fn;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Expr {
    Unit(MaybeType),
    Int(MaybeType, i64),
    EnvRef(String),
    Apply(MaybeType, Box<Expr>, Box<Expr>),
    Cond(MaybeType, Box<Expr>, Box<Expr>, Box<Expr>),
    Closure(MaybeType, String, MaybeType, Box<Expr>),
    Struct(MaybeType, Vec<(String, MaybeType, Expr)>),
    Discard(MaybeType),
    Match(MaybeType, Box<Expr>, Vec<(Expr, Expr)>),
    Let(MaybeType, String, MaybeType, Box<Expr>, Box<Expr>),
}

type In = crate::parser::preprocess::Out;

pub fn parse_expr(seq: Vec<In>) -> MaybeExpr {
    println!("\nParsing Expr seq: {:?}", seq);
    go(vec![Pat::Start], seq).into()
}

#[cfg(test)]
mod test;
