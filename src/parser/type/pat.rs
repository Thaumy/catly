use std::collections::BTreeSet;

use crate::parser::{BoxExt, VecExt};
use crate::parser::r#type::Type;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum Pat {
    Start,
    End,
    Err,

    Mark(char),
    Blank,

    //Type::IntType
    IntType,

    //Type::UnitType
    UnitType,

    DiscardType,//Type::DiscardType

    Char(char),
    CharSeq(String),

    //Type::TypeEnvRef
    TypeName(String),

    //Type::TypeApply
    TypeApply(Box<Pat>, Box<Pat>),

    Arrow,
    TypeClosurePara(String),
    TypeClosure(String, Box<Pat>),//Type::TypeClosure

    SumType(BTreeSet<Pat>),//Type::SumType

    LetName(String),
    LetNameWithType(String, Box<Pat>),
    LetNameWithTypeSeq(Vec<(String, Pat)>),
    ProductType(Vec<(String, Pat)>),//Type::ProductType
}

impl Pat {
    pub(crate) fn is_type(&self) -> bool {
        match self {
            Pat::IntType |
            Pat::UnitType |
            Pat::DiscardType |
            Pat::TypeName(_) |
            Pat::TypeApply(_, _) |
            Pat::TypeClosure(_, _) |
            Pat::SumType(_) |
            Pat::ProductType(_)
            => true,
            _ => false,
        }
    }
}

impl From<Pat> for Option<Type> {
    fn from(pat: Pat) -> Self {
        let r = match pat {
            Pat::IntType => Type::IntType,
            Pat::UnitType => Type::UnitType,
            Pat::DiscardType => Type::DiscardType,
            Pat::TypeName(n) => Type::TypeEnvRef(n),
            Pat::TypeApply(lhs, rhs) =>
                match (Self::from(*lhs), Self::from(*rhs)) {
                    (Some(lhs), Some(rhs)) =>
                        Type::TypeApply(
                            lhs.boxed(),
                            rhs.boxed(),
                        ),
                    _ => return None
                }

            Pat::TypeClosure(para, t) =>
                match Self::from(*t) {
                    Some(t) => Type::TypeClosure(
                        para,
                        t.boxed(),
                    ),
                    _ => return None
                },
            Pat::SumType(ts) => {
                type F = fn(Option<BTreeSet<Type>>, &Pat) -> Option<BTreeSet<Type>>;
                let f: F = |acc, t|
                    match (acc, Self::from(t.clone())) {
                        (Some(mut ts), Some(t)) => {
                            ts.insert(t);
                            Some(ts)
                        }
                        _ => None,
                    };
                let set = ts.iter().fold(Some(BTreeSet::new()), f);

                match set {
                    Some(set) => Type::SumType(set),
                    _ => return None,
                }
            }
            Pat::ProductType(vec) => {
                type LetNameWithType = (String, Type);
                type F = fn(Option<Vec<LetNameWithType>>, &(String, Pat)) -> Option<Vec<LetNameWithType>>;
                let f: F = |acc, (n, p)|
                    match (acc, Self::from(p.clone())) {
                        (Some(vec), Some(e)) =>
                            Some(vec.push_to_new((n.to_string(), e))),
                        _ => None,
                    };
                let vec = vec.iter().fold(Some(vec![]), f);

                match vec {
                    Some(vec) => Type::ProductType(vec),
                    _ => return None,
                }
            }
            _ => return None
        };
        Some(r)
    }
}
