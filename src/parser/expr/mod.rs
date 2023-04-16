use std::vec;

use crate::infra::alias::MaybeExpr;
use crate::parser::expr::pat::Pat;
use crate::parser::expr::r#fn::go;

mod r#fn;
mod pat;
pub mod r#type;
#[cfg(test)]
mod test;

type In = crate::parser::preprocess::Out;

pub fn parse_expr(seq: Vec<In>) -> MaybeExpr {
    println!("Parsing Expr seq: {seq:?}");
    go(vec![Pat::Start], seq).into()
}
