use std::collections::BTreeSet;
use std::ops::Deref;
use std::rc::Rc;

use crate::btree_set;
use crate::infra::btree_set::BtreeSetExt;
use crate::infra::option::OptionAnyExt;
use crate::infra::rc::RcAnyExt;
use crate::infra::vec::VecExt;
use crate::parser::r#type::r#type::OptType;
use crate::parser::r#type::r#type::Type;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pat {
    Start,
    End,
    Err,

    Mark(char),

    TypeName(String), // Type::NamelyType

    TypeApply(Rc<Pat>, Rc<Pat>), // Type::TypeApply

    Arrow,
    ClosureTypeHead(Rc<Pat>),
    ClosureType(Rc<Pat>, Rc<Pat>), // Type::ClosureType

    SumType(BTreeSet<Pat>), // Type::SumType

    LetName(Option<Rc<Pat>>, String),
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
                Self::from(i_t.deref().clone())?.rc(),
                Self::from(o_t.deref().clone())?.rc()
            ),

            Pat::SumType(s_s) => s_s
                .into_iter()
                .try_fold(btree_set![], |acc, t| {
                    let t: Self = t.into();
                    acc.chain_insert(t?).some()
                })
                .map(|set| Type::SumType(set))?,

            Pat::ProdType(p_v) => p_v
                .into_iter()
                .try_fold(vec![], |acc, (n, p)| {
                    let t: Self = p.into();
                    acc.chain_push((n, t?)).some()
                })
                .map(|vec| Type::ProdType(vec))?,
            _ => return None
        }
        .some()
    }
}
