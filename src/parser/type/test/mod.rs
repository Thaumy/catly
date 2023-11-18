use crate::lexer::lexical_analyze;
use crate::parser::r#type::parse_type;
use crate::parser::r#type::OptType;
use crate::pp::preprocess;

fn f(seq: &str) -> OptType {
    let preprocessed = preprocess(&seq);
    let tokens = lexical_analyze(preprocessed.as_str())?;
    parse_type(tokens.into_iter())
}

mod closure;
mod int;
mod namely;
mod prod;
mod sum;
mod unit;
