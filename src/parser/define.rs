use crate::parser::define::pat::Pat;
use crate::parser::define::r#fn::go;
use crate::parser::expr::Expr;
use crate::parser::r#type::Type;

mod pat;
mod r#fn;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Define {
    TypeDef(String, Type),
    ExprDef(String, Expr),
}

type In = crate::parser::preprocess::Out;

pub fn parse_define(seq: Vec<In>) -> Option<Define> {
    println!("\nParsing Define seq: {:?}", seq);
    go(vec![Pat::Start], seq).into()
}

#[cfg(test)]
mod test;
