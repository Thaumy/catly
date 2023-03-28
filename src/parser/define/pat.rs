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

    Blank,
    Mark(char),

    LetName(String),
    TypeName(String),

    Kw(Keyword),

    TypeDefHead(String),
    TypeDef(String, Type),

    ExprDefHead(String),
    ExprDef(String, Expr),
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
