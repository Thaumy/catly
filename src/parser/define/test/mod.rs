use crate::parser::define::{parse_define, Define};
use crate::parser::preprocess::preprocess;

fn f(seq: &str) -> Option<Define> {
    let seq = preprocess(&seq)?;
    parse_define(seq)
}

mod expr_def;
mod type_def;
