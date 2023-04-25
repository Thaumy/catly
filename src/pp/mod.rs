use crate::parser::keyword::Keyword;
use crate::pp::chunk::pp_chunk;
use crate::pp::comment::pp_comment;
use crate::pp::keyword::pp_keyword;
use crate::pp::merge_blank::pp_merge_blank;
use crate::pp::name::pp_name;
use crate::pp::r#const::pp_const;
use crate::pp::remove_blank::pp_remove_blank;

mod chunk;
mod comment;
mod r#const;
mod keyword;
mod merge_blank;
mod name;
mod remove_blank;

#[derive(Debug, Clone, PartialEq)]
pub enum Out {
    Symbol(char),
    LetName(String),
    TypeName(String),
    Kw(Keyword),

    IntValue(i64),
    UnitValue,
    DiscardValue
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
            In::DiscardValue => Out::DiscardValue
        }
    }
}

pub fn preprocess(seq: &str) -> Option<Vec<Out>> {
    let r = pp_comment(seq);
    let r = pp_merge_blank(&r);
    let r = pp_chunk(&r)?;
    let r = pp_keyword(&r);
    let r = pp_const(&r)?;
    let r = pp_name(&r)?;
    let r = pp_remove_blank(&r);
    let r = r
        .iter()
        .map(|x| Out::from(x.clone()))
        .collect();

    Some(r)
}
