use crate::lexer::Token;
use crate::parser::define::pat::Pat;
use crate::parser::define::r#fn::go;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::OptType;
use crate::parser::r#type::Type;

mod r#fn;
mod pat;
#[cfg(test)]
mod test;

#[derive(Clone, Debug, PartialEq)]
pub enum Define {
    TypeDef(String, Type),
    ExprDef(String, OptType, Expr)
}

pub fn parse_define(seq: Vec<Token>) -> Option<Define> {
    let r = go(vec![Pat::Start], seq.into_iter()).into();

    #[cfg(feature = "parser_log")]
    {
        let log = format!("{:8}{:>10} │ {r:?}", "[parsed]", "Define");
        println!("{log}");
    }

    r
}
