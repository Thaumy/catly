use std::vec;

use crate::lexer::Token;
use crate::parser::expr::pat::Pat;
use crate::parser::expr::r#fn::go;
use crate::parser::expr::r#type::OptExpr;

mod r#fn;
mod pat;
#[cfg(test)]
mod test;
pub mod r#type;

pub fn parse_expr<S>(seq: S) -> OptExpr
where
    S: Iterator<Item = Token> + Clone
{
    let r = go(vec![Pat::Start], seq).into();

    #[cfg(feature = "parser_log")]
    {
        let log = format!("{:8}{:>10} │ {r:?}", "[parsed]", "Expr");
        println!("{log}");
    }

    r
}
