use crate::parser::preprocess::blank::preprocess_blank;
use crate::parser::preprocess::comment::preprocess_comment;
use crate::parser::r#type::pat::Pat;
use crate::parser::r#type::r#fn::go;

mod pat;
mod r#fn;
mod test;
mod follow_pat;

// TODO
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[allow(dead_code)]
pub enum Type {
    IntType,
    UnitType,
    DiscardType,

    TypeEnvRef(String),
    TypeApply(Box<Type>, Box<Type>),
    TypeClosure(String, Box<Type>),
    SumType(Vec<Type>),
    ProductType(Vec<(String, Type)>),
}

pub fn parse_type(seq: &str) -> Option<Type> {
    println!("\nParsing seq: {:?}", seq);
    let seq = preprocess_comment(seq);
    let seq = preprocess_blank(&seq);
    Option::<Type>::from(go(&vec![Pat::Start], &seq))
}
