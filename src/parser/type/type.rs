use std::collections::BTreeSet;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    NamelyType(String),
    ClosureType(Box<Type>, Box<Type>),
    SumType(BTreeSet<Type>),
    ProdType(Vec<(String, Type)>),
    // Input type only
    PartialClosureType(Box<Type>)
}

impl Type {
    pub fn is_partial_type(&self) -> bool {
        match self {
            Type::NamelyType(_) => false,
            Type::ClosureType(i_t, o_t) =>
                i_t.is_partial_type() || o_t.is_partial_type(),
            Type::SumType(sum_set) => sum_set
                .iter()
                .any(|t| t.is_partial_type()),
            Type::ProdType(prod_vec) => prod_vec
                .iter()
                .any(|(_, t)| t.is_partial_type()),
            Type::PartialClosureType(..) => true
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::NamelyType(n) => f.write_str(&*format!("'{n}'")),
            Type::ClosureType(i_t, o_t) =>
                if let Type::ClosureType(..) |
                Type::PartialClosureType(..) = i_t.clone().deref()
                {
                    f.write_str(&*format!("({i_t:?}) -> {o_t:?}"))
                } else {
                    f.write_str(&*format!("{i_t:?} -> {o_t:?}"))
                },
            Type::SumType(s) =>
                f.write_str(&*format!("SumType{s:?}")),
            Type::ProdType(v) =>
                f.write_str(&*format!("ProdType{v:?}")),

            // Input type only
            Type::PartialClosureType(i_t) =>
                if let Type::ClosureType(..) |
                Type::PartialClosureType(..) = i_t.clone().deref()
                {
                    f.write_str(&*format!("({i_t:?}) -> ?"))
                } else {
                    f.write_str(&*format!("{i_t:?} -> ?"))
                },
        }
    }
}
