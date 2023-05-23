use crate::lexer::lexical_analyze;
use crate::parser::define::{parse_define, Define};
use crate::pp::preprocess;

fn f(seq: &str) -> Option<Define> {
    let preprocessed = preprocess(&seq);
    let tokens = lexical_analyze(preprocessed.as_str())?;
    parse_define(tokens)
}

mod expr_def;
mod type_def;
