use crate::parser::define::Define;
use crate::parser::expr::Expr;
use crate::parser::keyword::Keyword;
use crate::parser::r#type::Type;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Pat {
    Start,
    End,
    Err,

    Mark(char),
    Kw(Keyword),

    LetName(String),
    TypeName(String),

    TypeDefHead(String),
    TypeDef(String, Type),// Define::TypeDef

    ExprDefHead(String),
    ExprDef(String, Expr),// Define::ExprDef
}

impl From<Pat> for Option<Define> {
    fn from(pat: Pat) -> Self {
        let r = match pat {
            Pat::TypeDef(d, t) => Define::TypeDef(d, t),
            Pat::ExprDef(d, e) => Define::ExprDef(d, e),
            _ => return None
        };
        Some(r)
    }
}
