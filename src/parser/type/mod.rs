use crate::parser::r#type::pat::Pat;
use crate::parser::r#type::r#fn::go;
use crate::parser::r#type::r#type::MaybeType;

mod r#fn;
mod pat;
#[cfg(test)]
mod test;
pub mod r#type;

type In = crate::pp::Out;

pub fn parse_type(seq: Vec<In>) -> MaybeType {
    let r = go(vec![Pat::Start], seq).into();

    if cfg!(feature = "parser_log") {
        let log = format!("{:8}{:>10} │ {r:?}", "[parsed]", "Type");
        println!("{log}");
    }

    r
}
