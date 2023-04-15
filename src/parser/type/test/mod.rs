use crate::infra::alias::MaybeType;
use crate::parser::preprocess::preprocess;
use crate::parser::r#type::parse_type;

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
