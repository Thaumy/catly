use std::collections::BTreeSet;

use crate::infra::alias::MaybeType;
use crate::infra::iter::maybe_fold;
use crate::infra::option::AnyExt;
use crate::infra::r#box::Ext as BoxAnyExt;
use crate::infra::btree_set::Ext;
use crate::infra::vec::Ext as VecAnyExt;
use crate::parser::r#type::r#type::Type;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pat {
    Start,
    End,
    Err,

    Mark(char),

    TypeName(String), // Type::NamelyType

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
            Pat::TypeName(..) |
            Pat::TypeApply(..) |
            Pat::ClosureType(..) |
            Pat::SumType(..) |
            Pat::ProdType(..) => true,
            _ => false
        }
    }
}

impl From<Pat> for MaybeType {
    fn from(pat: Pat) -> Self {
        let r = match pat {
            Pat::TypeName(n) => Type::NamelyType(n),

            Pat::ClosureType(i, o) => {
                match (Self::from(*i), Self::from(*o)) {
                    (Some(i), Some(o)) =>
                        Type::ClosureType(i.boxed(), o.boxed()),
                    _ => return None
                }
            }
            Pat::SumType(ts) => {
                let set = maybe_fold(
                    ts.iter(),
                    BTreeSet::new(),
                    |acc, t| {
                        let it = (t.clone().into(): MaybeType)?;
                        acc.chain_insert(it).some()
                    }
                );

                match set {
                    Some(set) => Type::SumType(set),
                    _ => return None
                }
            }
            Pat::ProdType(vec) => {
                let vec =
                    maybe_fold(vec.iter(), vec![], |acc, (n, p)| {
                        let it = (p.clone().into(): MaybeType)
                            .map(|e| (n.to_string(), e))?;
                        acc.chain_push(it).some()
                    });

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
