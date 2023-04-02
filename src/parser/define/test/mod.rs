use crate::parser::define::{Define, parse_define};
use crate::parser::preprocess::preprocess;

fn f(seq: &str) -> Option<Define> {
    let seq = preprocess(&seq)?;
    parse_define(seq)
}

mod type_def;
mod expr_def;
