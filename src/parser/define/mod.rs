use crate::infra::alias::MaybeType;
use crate::parser::define::pat::Pat;
use crate::parser::define::r#fn::go;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::Type;

mod r#fn;
mod pat;
#[cfg(test)]
mod test;

#[derive(Debug, PartialEq)]
pub enum Define {
    TypeDef(String, Type),
    ExprDef(String, MaybeType, Expr)
}

type In = crate::parser::preprocess::Out;

pub fn parse_define(seq: Vec<In>) -> Option<Define> {
    println!("Parsing Define seq: {seq:?}");
    go(vec![Pat::Start], seq).into()
}
