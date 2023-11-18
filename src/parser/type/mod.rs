use crate::lexer::Token;
use crate::parser::r#type::pat::Pat;
use crate::parser::r#type::r#fn::go;

mod r#fn;
mod pat;
#[cfg(test)]
mod test;
mod r#type;

pub use r#type::*;

pub fn parse_type<S>(seq: S) -> OptType
where
    S: Iterator<Item = Token> + Clone
{
    let r = go(vec![Pat::Start], seq).into();

    #[cfg(feature = "parser_log")]
    {
        let log = format!("{:8}{:>10} │ {r:?}", "[parsed]", "Type");
        println!("{log}");
    }

    r
}
