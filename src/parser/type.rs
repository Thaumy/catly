use std::collections::BTreeSet;

use crate::parser::Either;
use crate::parser::keyword::Keyword;
use crate::parser::r#type::pat::Pat;
use crate::parser::r#type::r#fn::go;

mod pat;
mod r#fn;
mod follow_pat;

// TODO
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum Type {
    IntType,
    UnitType,
    DiscardType,

    TypeEnvRef(String),
    TypeApply(Box<Type>, Box<Type>),

    //different meaning in type define and type annotation
    TypeClosure(String, Box<Type>),

    SumType(BTreeSet<Type>),
    ProductType(Vec<(String, Type)>),
}

pub fn parse_type(seq: Vec<Either<char, Keyword>>) -> Option<Type> {
    println!("\nParsing seq: {:?}", seq);
    Option::<Type>::from(go(&vec![Pat::Start], seq))
}

mod test;
