use crate::infra::option::OptionAnyExt;
use crate::parser::define::{Define, In};
use crate::parser::expr::r#type::Expr;
use crate::parser::keyword::Keyword;
use crate::parser::r#type::r#type::OptType;
use crate::parser::r#type::r#type::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum Pat {
    Start,
    End,
    Err,

    Mark(char),
    Kw(Keyword),

    AnyIn(In),
    AnyInSeq(Vec<In>),
    Type(Type),
    LetName(OptType, String),

    TypeName(String),

    TypeDefHead(String),
    TypeDef(String, Type), // Define::TypeDef

    ExprDefHead(OptType, String),
    ExprDef(String, OptType, Expr) // Define::ExprDef
}

impl From<Pat> for Option<Define> {
    fn from(pat: Pat) -> Self {
        match pat {
            Pat::TypeDef(d, t) => Define::TypeDef(d, t),
            Pat::ExprDef(d, t, e) => Define::ExprDef(d, t, e),
            _ => return None
        }
        .some()
    }
}
