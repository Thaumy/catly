use std::collections::BTreeSet;

use crate::infra::alias::MaybeType;
use crate::infra::r#box::Ext;
use crate::maybe_fold_to;
use crate::parser::r#type::Type;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pat {
    Start,
    End,
    Err,

    Mark(char),

    TypeName(String), // Type::TypeEnvRef

    TypeApply(Box<Pat>, Box<Pat>), // Type::TypeApply

    Arrow,
    ClosureTypeHead(Box<Pat>),
    ClosureType(Box<Pat>, Box<Pat>), // Type::ClosureType

    SumType(BTreeSet<Pat>), // Type::SumType

    LetName(Option<Box<Pat>>, String),
    TypedLetNameSeq(Vec<(String, Pat)>),
    ProdType(Vec<(String, Pat)>) // Type::ProdType
}

impl Pat {
    pub(crate) fn is_type(&self) -> bool {
        match self {
            Pat::TypeName(_) |
            Pat::TypeApply(_, _) |
            Pat::ClosureType(_, _) |
            Pat::SumType(_) |
            Pat::ProdType(_) => true,
            _ => false
        }
    }
}

impl From<Pat> for MaybeType {
    fn from(pat: Pat) -> Self {
        let r = match pat {
            Pat::TypeName(n) => Type::TypeEnvRef(n),

            Pat::ClosureType(i, o) => {
                match (Self::from(*i), Self::from(*o)) {
                    (Some(i), Some(o)) =>
                        Type::ClosureType(i.boxed(), o.boxed()),
                    _ => return None
                }
            }
            Pat::SumType(ts) => {
                let set = maybe_fold_to!(
                    ts.iter(),
                    BTreeSet::new(),
                    insert,
                    |t: &Pat| t.clone().into()
                );

                match set {
                    Some(set) => Type::SumType(set),
                    _ => return None
                }
            }
            Pat::ProdType(vec) => {
                let f = |(n, p): &(String, Pat)| {
                    (p.clone().into(): MaybeType)
                        .map(|e| (n.to_string(), e))
                };

                let vec = maybe_fold_to!(vec.iter(), vec![], push, f);

                match vec {
                    Some(vec) => Type::ProdType(vec),
                    _ => return None
                }
            }
            _ => return None
        };
        Some(r)
    }
}
