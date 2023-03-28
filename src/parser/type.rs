use std::collections::BTreeSet;

use crate::parser::infra::MaybeType;
use crate::parser::preprocess::Out;
use crate::parser::r#type::pat::Pat;
use crate::parser::r#type::r#fn::go;

mod pat;
mod r#fn;

// TODO
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum Type {
    TypeEnvRef(String),
    ClosureType(Box<Type>, Box<Type>),
    SumType(BTreeSet<Type>),
    ProductType(Vec<(String, Type)>),
}

pub fn parse_type(seq: Vec<Out>) -> MaybeType {
    println!("\nParsing seq: {:?}", seq);
    Option::<Type>::from(go(&vec![Pat::Start], seq))
}

mod test;
