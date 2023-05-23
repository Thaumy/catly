use crate::lexer::lexical_analyze;
use crate::parser::expr::parse_expr;
use crate::parser::expr::r#type::OptExpr;
use crate::pp::preprocess;

pub fn f(seq: &str) -> OptExpr {
    let preprocessed = preprocess(&seq);
    let tokens = lexical_analyze(preprocessed.as_str())?;
    parse_expr(tokens.into_iter())
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
