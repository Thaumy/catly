use crate::lexer::lexical_analyze;
use crate::parser::ast::parse_ast;
use crate::parser::define::Define;
use crate::pp::preprocess;

fn f(seq: &str) -> Option<Vec<Define>> {
    let preprocessed = preprocess(&seq);
    let tokens = lexical_analyze(preprocessed.as_str())?;
    parse_ast(tokens)
}

mod part1;
mod part2;
mod part3;
