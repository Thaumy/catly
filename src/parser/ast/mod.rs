use crate::parser::ast::r#fn::{
    parse_to_defines,
    split_to_top_levels
};
use crate::parser::define::Define;

mod r#fn;
#[cfg(test)]
mod test;

type In = crate::pp::Out;

pub fn parse_ast(seq: Vec<In>) -> Option<Vec<Define>> {
    let seq = split_to_top_levels(seq);
    let r = parse_to_defines(seq);

    if cfg!(feature = "parser_log") {
        let log = format!("{:8}{:>10} │ {r:?}", "[parsed]", "AST");
        println!("{log}");
    }

    r
}
