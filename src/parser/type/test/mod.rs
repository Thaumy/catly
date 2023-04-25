use crate::parser::r#type::parse_type;
use crate::parser::r#type::r#type::MaybeType;
use crate::pp::preprocess;

fn f(seq: &str) -> MaybeType {
    let seq = preprocess(&seq)?;
    parse_type(seq)
}

mod closure;
mod int;
mod namely;
mod prod;
mod sum;
mod unit;
