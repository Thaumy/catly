use std::collections::BTreeSet;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

use crate::infra::btree_set::Ext as BtTreeAnyExt;
use crate::infra::option::AnyExt;
use crate::infra::r#box::Ext as BoxAnyExt;
use crate::infra::vec::Ext as VecAnyExt;
use crate::parser::r#type::r#type::Type as CtType;

pub type MaybeType = Option<Type>;

pub type ProdField = (String, Type);

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Type {
    NamelyType(String),
    ClosureType(Box<Type>, Box<Type>),
    SumType(BTreeSet<Type>),
    ProdType(Vec<ProdField>)
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::NamelyType(n) => f.write_str(&*format!("'{n}'")),
            Type::ClosureType(i_t, o_t) =>
                if let Type::ClosureType(..) = i_t.clone().deref() {
                    f.write_str(&*format!("({i_t:?}) -> {o_t:?}"))
                } else {
                    f.write_str(&*format!("{i_t:?} -> {o_t:?}"))
                },
            Type::SumType(s) =>
                f.write_str(&*format!("SumType{s:?}")),
            Type::ProdType(v) =>
                f.write_str(&*format!("ProdType{v:?}")),
        }
    }
}

impl From<CtType> for MaybeType {
    fn from(value: CtType) -> Self {
        match value {
            CtType::NamelyType(t_n) => Type::NamelyType(t_n),

            CtType::ClosureType(i_t, o_t) => Type::ClosureType(
                Self::from(*i_t)?.boxed(),
                Self::from(*o_t)?.boxed()
            ),

            CtType::SumType(s_s) => s_s
                .iter()
                .try_fold(BTreeSet::new(), |acc, t| {
                    let t: Self = t.clone().into();
                    acc.chain_insert(t?).some()
                })
                .map(|set| Type::SumType(set))?,

            CtType::ProdType(p_v) => p_v
                .iter()
                .try_fold(vec![], |acc, (n, t)| {
                    let n = n.to_string();
                    let t: Self = t.clone().into();
                    acc.chain_push((n, t?)).some()
                })
                .map(|vec| Type::ProdType(vec))?,

            CtType::PartialClosureType(_) => return None
        }
        .some()
    }
}
