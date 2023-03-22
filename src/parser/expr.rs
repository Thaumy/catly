use std::vec;

use crate::parser::expr::pat::Pat;
use crate::parser::expr::r#fn::go;
use crate::parser::infra::Either;
use crate::parser::keyword::Keyword;

mod pat;
mod r#fn;

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

pub fn parse_expr(seq: Vec<Either<char, Keyword>>) -> Option<Expr> {
    println!("\nParsing seq: {:?}", seq);
    Option::<Expr>::from(go(&vec![Pat::Start], seq))
}

mod test;
