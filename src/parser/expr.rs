use std::vec;

use crate::parser::expr::pat::Pat;
use crate::parser::expr::r#fn::go;
use crate::parser::preprocess::blank::preprocess_blank;
use crate::parser::preprocess::comment::preprocess_comment;
use crate::parser::preprocess::keyword::preprocess_keyword;

mod pat;
mod r#fn;
mod follow_pat;

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

pub fn parse_expr(seq: &str) -> Option<Expr> {
    println!("\nParsing seq: {:?}", seq);
    let seq = preprocess_comment(seq);
    let seq = preprocess_blank(&seq);
    let seq = preprocess_keyword(&seq);
    Option::<Expr>::from(go(&vec![Pat::Start], seq))
}

mod test;
