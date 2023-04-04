use std::collections::BTreeSet;

use crate::infra::alias::MaybeType;
use crate::parser::r#type::pat::Pat;
use crate::parser::r#type::r#fn::go;

mod r#fn;
mod pat;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    TypeEnvRef(String),
    ClosureType(Box<Type>, Box<Type>),
    SumType(BTreeSet<Type>),
    ProdType(Vec<(String, Type)>)
}

type In = crate::parser::preprocess::Out;

pub fn parse_type(seq: Vec<In>) -> MaybeType {
    println!("\nParsing Type seq: {:?}", seq);
    go(vec![Pat::Start], seq).into()
}

#[cfg(test)]
mod test;
