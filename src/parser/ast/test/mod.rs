use crate::parser::ast::parse_ast;
use crate::parser::define::Define;
use crate::pp::preprocess;

fn f(seq: &str) -> Option<Vec<Define>> {
    let seq = preprocess(&seq)?;
    parse_ast(seq)
}

mod part1;
mod part2;
mod part3;
