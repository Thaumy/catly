use std::vec;

use crate::parser::expr::pat::Pat;
use crate::parser::expr::r#fn::go;
use crate::parser::infra::{Either, MaybeExpr, MaybeType};
use crate::parser::keyword::Keyword;
use crate::parser::preprocess::Out;

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
    Discard,
    Match(MaybeType, Box<Expr>, Vec<(Expr, Expr)>),
    Let(MaybeType, String, MaybeType, Box<Expr>, Box<Expr>),
}

pub fn parse_expr(seq: Vec<Out>) -> MaybeExpr {
    println!("\nParsing seq: {:?}", seq);
    Option::<Expr>::from(go(&vec![Pat::Start], seq))
}

mod test;
