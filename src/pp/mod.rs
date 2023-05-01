use crate::infra::option::OptionAnyExt;
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
    fn from(value: In) -> Self {
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
    let r = pp_keyword(r.iter());
    let r = pp_const(r.iter())?;
    let r = pp_name(r.iter())?;
    let r = pp_remove_blank(r.iter());

    r.iter()
        .map(|x| x.clone().into())
        .collect::<Vec<Out>>()
        .some()
}

pub trait FollowExt<T> {
    fn is_expr_end_pat(&self) -> bool;
    fn is_type_end_pat(&self) -> bool;
}

impl FollowExt<Out> for Option<Out> {
    fn is_expr_end_pat(&self) -> bool {
        match self {
            None |
            Some(Out::Symbol(')')) |// ..
            Some(Out::Symbol('}')) |// Struct
            Some(Out::Symbol(',')) |// Assign (Struct, Let
            Some(Out::Symbol('|')) |// Match
            Some(Out::Symbol('=')) |// Assign (Struct, Let
            Some(Out::Kw(_))// 这意味着`最近可立即归约`的语言构造具备更高的结合优先级
            => true,
            _ => false,
        }
    }
    fn is_type_end_pat(&self) -> bool {
        match self {
            None |
            Some(Out::Symbol(')')) |
            Some(Out::Symbol('}')) |
            Some(Out::Symbol(',')) |
            Some(Out::Symbol('=')) |
            Some(Out::Kw(_)) |
            Some(Out::LetName(_)) |
            Some(Out::TypeName(_)) |
            Some(Out::IntValue(_)) |
            Some(Out::UnitValue) |
            Some(Out::DiscardValue) => true,

            _ => false
        }
    }
}
