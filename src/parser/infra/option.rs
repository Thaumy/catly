use crate::parser::preprocess::Out;

pub trait Ext {
    fn is_end_pat(&self) -> bool;
}

impl Ext for Option<Out> {
    fn is_end_pat(&self) -> bool {
        match self {
            None |
            Some(Out::Symbol(')')) |// ..
            Some(Out::Symbol('}')) |// Struct
            Some(Out::Symbol(',')) |// Struct, Let
            Some(Out::Symbol('|')) |// Match
            Some(Out::Kw(_))// 这意味着 最近可立即归约 的语言构造具备更高的结合优先级
            => true,
            _ => false,
        }
    }
}