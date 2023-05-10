use crate::parser::expr::parse_expr;
use crate::parser::expr::r#type::OptExpr;
use crate::pp::preprocess;

pub fn f(seq: &str) -> OptExpr {
    let seq = preprocess(&seq)?;
    parse_expr(seq.into_iter())
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
