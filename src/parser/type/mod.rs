use crate::infra::alias::MaybeType;
use crate::parser::r#type::pat::Pat;
use crate::parser::r#type::r#fn::go;

mod r#fn;
mod pat;
pub mod r#type;
#[cfg(test)]
mod test;

type In = crate::parser::preprocess::Out;

pub fn parse_type(seq: Vec<In>) -> MaybeType {
    println!("Parsing Type seq: {seq:?}");
    go(vec![Pat::Start], seq).into()
}
