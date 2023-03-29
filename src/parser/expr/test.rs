use crate::parser::expr::parse_expr;
use crate::parser::infra::alias::MaybeExpr;
use crate::parser::preprocess::preprocess;

pub fn f(seq: &str) -> MaybeExpr {
    let seq = preprocess(&seq)?;
    parse_expr(seq)
}

mod unit;
mod int;
mod env_ref;
mod apply;
mod cond;
mod closure;
mod r#struct;
mod r#match;
mod r#let;
