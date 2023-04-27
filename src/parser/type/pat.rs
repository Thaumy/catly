use std::collections::BTreeSet;

use crate::btree_set;
use crate::infra::btree_set::Ext;
use crate::infra::option::AnyExt;
use crate::infra::r#box::Ext as BoxAnyExt;
use crate::infra::vec::Ext as VecAnyExt;
use crate::parser::r#type::r#type::OptType;
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

impl From<Pat> for OptType {
    fn from(pat: Pat) -> Self {
        match pat {
            Pat::TypeName(t_n) => Type::NamelyType(t_n),

            Pat::ClosureType(i_t, o_t) => Type::ClosureType(
                Self::from(*i_t)?.boxed(),
                Self::from(*o_t)?.boxed()
            ),

            Pat::SumType(s_s) => s_s
                .iter()
                .try_fold(btree_set![], |acc, t| {
                    let t: Self = t.clone().into();
                    acc.chain_insert(t?).some()
                })
                .map(|set| Type::SumType(set))?,

            Pat::ProdType(p_v) => p_v
                .iter()
                .try_fold(vec![], |acc, (n, p)| {
                    let n = n.to_string();
                    let t: Self = p.clone().into();
                    acc.chain_push((n, t?)).some()
                })
                .map(|vec| Type::ProdType(vec))?,
            _ => return None
        }
        .some()
    }
}
