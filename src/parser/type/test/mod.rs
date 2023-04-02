use crate::parser::infra::alias::MaybeType;
use crate::parser::preprocess::preprocess;
use crate::parser::r#type::{parse_type};

fn f(seq: &str) -> MaybeType {
    let seq = preprocess(&seq)?;
    parse_type(seq)
}

mod unit;
mod int;
mod env_ref;
mod closure;
mod sum;
mod prod;
