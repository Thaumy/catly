use r#fn::*;

use crate::lexer::Token;
use crate::parser::define::Define;

mod r#fn;
#[cfg(test)]
mod test;

pub fn parse_ast(seq: Vec<Token>) -> Option<Vec<Define>> {
    let seq = split_to_top_levels(seq);
    let r = parse_to_defines(seq);

    #[cfg(feature = "parser_log")]
    {
        let log = format!("{:8}{:>10} │ {r:?}", "[parsed]", "AST");
        println!("{log}");
    }

    r
}
