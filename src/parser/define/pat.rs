use crate::infra::WrapOption;
use crate::lexer::Token;
use crate::parser::define::Define;
use crate::parser::expr::r#type::Expr;
use crate::parser::keyword::Keyword;
use crate::parser::r#type::OptType;
use crate::parser::r#type::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum Pat {
    Start,
    End,
    Err,

    Mark(char),
    Kw(Keyword),

    AnyToken(Token),
    AnyTokenSeq(Vec<Token>),
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
            Pat::TypeDef(n, t) => Define::TypeDef(n, t),
            Pat::ExprDef(n, t, e) => Define::ExprDef(n, t, e),
            _ => return None
        }
        .wrap_some()
    }
}
