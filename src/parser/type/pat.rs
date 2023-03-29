use std::collections::BTreeSet;

use crate::maybe_fold;
use crate::parser::infra::alias::MaybeType;
use crate::parser::infra::r#box::Ext;
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

    //Type::TypeEnvRef
    TypeName(String),

    //Type::TypeApply
    TypeApply(Box<Pat>, Box<Pat>),

    Arrow,
    ClosureTypeHead(Box<Pat>),
    ClosureType(Box<Pat>, Box<Pat>),//Type::ClosureType

    SumType(BTreeSet<Pat>),//Type::SumType

    LetName(String),
    LetNameWithType(String, Box<Pat>),
    LetNameWithTypeSeq(Vec<(String, Pat)>),
    ProductType(Vec<(String, Pat)>),//Type::ProductType
}

impl Pat {
    pub(crate) fn is_type(&self) -> bool {
        match self {
            Pat::TypeName(_) |
            Pat::TypeApply(_, _) |
            Pat::ClosureType(_, _) |
            Pat::SumType(_) |
            Pat::ProductType(_)
            => true,
            _ => false,
        }
    }
}

impl From<Pat> for MaybeType {
    fn from(pat: Pat) -> Self {
        let r = match pat {
            Pat::TypeName(n) => Type::TypeEnvRef(n),

            Pat::ClosureType(para, t) =>
                match (Self::from(*para), Self::from(*t)) {
                    (Some(para), Some(t)) => Type::ClosureType(
                        para.boxed(),
                        t.boxed(),
                    ),
                    _ => return None
                },
            Pat::SumType(ts) => {
                let set = maybe_fold!(
                    ts.iter(),
                    BTreeSet::new(),
                    insert,
                    |t: &Pat| t.clone().into()
                );

                match set {
                    Some(set) => Type::SumType(set),
                    _ => return None,
                }
            }
            Pat::ProductType(vec) => {
                let f = |(n, p): &(String, Pat)|
                    (p.clone().into(): MaybeType).map(|e| (n.to_string(), e));

                let vec = maybe_fold!(
                    vec.iter(),
                    vec![],
                    push,
                    f
                );

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
