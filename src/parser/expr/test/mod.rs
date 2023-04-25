use crate::parser::expr::parse_expr;
use crate::parser::expr::r#type::MaybeExpr;
use crate::pp::preprocess;

pub fn f(seq: &str) -> MaybeExpr {
    let seq = preprocess(&seq)?;
    parse_expr(seq)
}

mod apply;
mod closure;
mod cond;
mod env_ref;
mod int;
mod r#let;
mod r#match;
mod r#struct;
mod unit;
