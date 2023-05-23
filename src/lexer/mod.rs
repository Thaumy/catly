use crate::infra::option::OptionAnyExt;
use crate::lexer::chunk::lexer_chunk;
use crate::lexer::keyword::lexer_keyword;
use crate::lexer::literal::lexer_literal;
use crate::lexer::name::lexer_name;
use crate::lexer::remove_blank::lexer_remove_blank;
use crate::parser::keyword::Keyword;

mod chunk;
mod keyword;
mod literal;
mod name;
mod remove_blank;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Symbol(char),
    LetName(String),
    TypeName(String),
    Kw(Keyword),

    IntValue(i64),
    UnitValue,
    DiscardValue
}

type In = name::Out;

impl From<In> for Token {
    fn from(value: In) -> Self {
        match value {
            In::Symbol(c) => Token::Symbol(c),
            In::LetName(n) => Token::LetName(n),
            In::TypeName(n) => Token::TypeName(n),
            In::Kw(kw) => Token::Kw(kw),
            In::IntValue(i) => Token::IntValue(i),
            In::UnitValue => Token::UnitValue,
            In::DiscardValue => Token::DiscardValue
        }
    }
}

pub fn lexical_analyze(preprocessed: &str) -> Option<Vec<Token>> {
    let r = lexer_chunk(preprocessed)?;
    let r = lexer_keyword(r.into_iter());
    let r = lexer_literal(r.into_iter())?;
    let r = lexer_name(r.into_iter())?;
    let r = lexer_remove_blank(r.into_iter());

    r.into_iter()
        .map(|x| x.into())
        .collect::<Vec<Token>>()
        .some()
}

pub trait FollowExt<T> {
    fn is_expr_end_pat(&self) -> bool;
    fn is_type_end_pat(&self) -> bool;
}

impl FollowExt<Token> for Option<Token> {
    fn is_expr_end_pat(&self) -> bool {
        matches!(
            self,
            None |
            Some(Token::Symbol(')')) |// ..
            Some(Token::Symbol('}')) |// Struct
            Some(Token::Symbol(',')) |// Assign (Struct, Let
            Some(Token::Symbol('|')) |// Match
            Some(Token::Symbol('=')) |// Assign (Struct, Let
            Some(Token::Kw(_)) // 这意味着`最近可立即归约`的语言构造具备更高的结合优先级
        )
    }
    fn is_type_end_pat(&self) -> bool {
        matches!(
            self,
            None | Some(Token::Symbol(')')) |
                Some(Token::Symbol('}')) |
                Some(Token::Symbol(',')) |
                Some(Token::Symbol('=')) |
                Some(Token::Kw(_)) |
                Some(Token::LetName(_)) |
                Some(Token::TypeName(_)) |
                Some(Token::IntValue(_)) |
                Some(Token::UnitValue) |
                Some(Token::DiscardValue)
        )
    }
}
