use std::vec;

use crate::infra::alias::MaybeExpr;
use crate::parser::expr::pat::Pat;
use crate::parser::expr::r#fn::go;

mod r#fn;
mod pat;
#[cfg(test)]
mod test;
pub mod r#type;

type In = crate::parser::preprocess::Out;

pub fn parse_expr(seq: Vec<In>) -> MaybeExpr {
    let r = go(vec![Pat::Start], seq).into();
    println!("{:8}{:>10} │ {r:?}", "[parsed]", "Expr");
    r
}
