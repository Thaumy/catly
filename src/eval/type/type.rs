use std::collections::BTreeSet;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::rc::Rc;

use crate::infra::BtreeSetExt;
use crate::infra::VecExt;
use crate::infra::WrapOption;
use crate::infra::WrapRc;
use crate::parser::r#type::Type as CtType;

pub type OptType = Option<Type>;

pub type ProdField = (String, Type);

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Type {
    NamelyType(String),
    ClosureType(Rc<Type>, Rc<Type>),
    SumType(BTreeSet<Type>),
    ProdType(Vec<ProdField>)
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::NamelyType(n) => f.write_str(&format!("'{n}'")),
            Type::ClosureType(i_t, o_t) =>
                if let Type::ClosureType(..) = i_t.clone().deref() {
                    f.write_str(&format!("({i_t:?}) -> {o_t:?}"))
                } else {
                    f.write_str(&format!("{i_t:?} -> {o_t:?}"))
                },
            Type::SumType(s) => f.write_str(&format!("SumType{s:?}")),
            Type::ProdType(v) =>
                f.write_str(&format!("ProdType{v:?}")),
        }
    }
}

impl From<CtType> for OptType {
    fn from(value: CtType) -> Self {
        match value {
            CtType::NamelyType(t_n) => Type::NamelyType(t_n),

            CtType::ClosureType(i_t, o_t) => Type::ClosureType(
                Self::from(i_t.deref().clone())?.wrap_rc(),
                Self::from(o_t.deref().clone())?.wrap_rc()
            ),

            CtType::SumType(s_s) => s_s
                .into_iter()
                .try_fold(BTreeSet::new(), |acc, t| {
                    let t: Self = t.into();
                    acc.chain_insert(t?)
                        .wrap_some()
                })
                .map(Type::SumType)?,

            CtType::ProdType(p_v) => p_v
                .into_iter()
                .try_fold(vec![], |acc, (n, t)| {
                    let t: Self = t.into();
                    acc.chain_push((n, t?))
                        .wrap_some()
                })
                .map(Type::ProdType)?,

            CtType::PartialClosureType(_) => return None
        }
        .wrap_some()
    }
}
