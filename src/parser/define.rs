use crate::parser::define::pat::Pat;
use crate::parser::define::r#fn::go;
use crate::parser::expr::Expr;
use crate::parser::preprocess::Out;
use crate::parser::r#type::Type;

mod pat;
mod r#fn;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Define {
    TypeDef(String, Type),
    ExprDef(String, Expr),
}

pub fn parse_define(seq: Vec<Out>) -> Option<Define> {
    println!("\nParsing seq: {:?}", seq);
    Option::<Define>::from(go(vec![Pat::Start], seq))
}

mod test;
