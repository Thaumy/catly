use crate::parser::r#type::parse_type;
use crate::parser::r#type::r#type::OptType;
use crate::pp::preprocess;

fn f(seq: &str) -> OptType {
    let seq = preprocess(&seq)?;
    parse_type(seq.into_iter())
}

mod closure;
mod int;
mod namely;
mod prod;
mod sum;
mod unit;
