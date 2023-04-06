use crate::parser::ast::r#fn::{
    parse_to_defines,
    split_to_top_levels
};
use crate::parser::define::Define;

mod r#fn;

type In = crate::parser::preprocess::Out;

pub fn parse_ast(seq: Vec<In>) -> Option<Vec<Define>> {
    let seq = split_to_top_levels(seq);
    let r = parse_to_defines(seq);
    println!("AST: {:?}", r);
    r
}

#[cfg(test)]
mod test;