use std::collections::BTreeSet;

use crate::infra::alias::MaybeType;
use crate::parser::r#type::pat::Pat;
use crate::parser::r#type::r#fn::go;

mod r#fn;
mod pat;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    TypeEnvRef(String),
    // 对 Closure 的类型信息缺失类型设计可以扩展到类型构造器
    ClosureType(Option<Box<Type>>, Option<Box<Type>>),
    SumType(BTreeSet<Type>),
    ProdType(Vec<(String, Type)>)
}

type In = crate::parser::preprocess::Out;

pub fn parse_type(seq: Vec<In>) -> MaybeType {
    println!("\nParsing Type seq: {:?}", seq);
    go(vec![Pat::Start], seq).into()
}

#[cfg(test)]
mod test;
