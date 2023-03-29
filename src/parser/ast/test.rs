use crate::parser::ast::parse_ast;
use crate::parser::define::Define;
use crate::parser::preprocess::preprocess;

fn f(seq: &str) -> Option<Vec<Define>> {
    let seq = preprocess(&seq)?;
    parse_ast(seq)
}

mod part1;
mod part2;
