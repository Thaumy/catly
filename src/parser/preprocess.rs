use crate::parser::keyword::Keyword;
use crate::parser::preprocess::blank::preprocess_blank;
use crate::parser::preprocess::chunk::preprocess_chunk;
use crate::parser::preprocess::comment::preprocess_comment;
use crate::parser::preprocess::keyword::preprocess_keyword;
use crate::parser::preprocess::name::preprocess_name;
use crate::parser::preprocess::r#const::preprocess_const;

pub mod keyword;
pub mod comment;
pub mod blank;
pub mod name;
pub mod chunk;
pub mod r#const;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Out {
    Symbol(char),
    LetName(String),
    TypeName(String),
    Kw(Keyword),

    IntValue(i64),
    UnitValue,
    DiscardValue,
}

type In = name::Out;

impl From<In> for Out {
    fn from(value: name::Out) -> Self {
        match value {
            In::Symbol(c) => Out::Symbol(c),
            In::LetName(n) => Out::LetName(n),
            In::TypeName(n) => Out::TypeName(n),
            In::Kw(kw) => Out::Kw(kw),
            In::IntValue(i) => Out::IntValue(i),
            In::UnitValue => Out::UnitValue,
            In::DiscardValue => Out::DiscardValue,
        }
    }
}

pub fn preprocess(seq: &str) -> Option<Vec<Out>> {
    let r = preprocess_comment(seq);
    let r = preprocess_blank(&r);
    let r = preprocess_chunk(&r)?;
    let r = preprocess_keyword(&r);
    let r = preprocess_const(&r)?;
    let r = preprocess_name(&r)?;
    let r = r
        .iter()
        .map(|x| Out::from(x.clone()))
        .collect();

    Some(r)
}