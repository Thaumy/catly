use std::collections::BTreeSet;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

pub type RtProdField = (String, RtType);

#[derive(Clone)]
pub enum RtType {
    NamelyType(String),
    ClosureType(Box<RtType>, Box<RtType>),
    SumType(BTreeSet<RtType>),
    ProdType(Vec<RtProdField>)
}

impl Debug for RtType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RtType::NamelyType(n) => f.write_str(&*format!("'{n}'")),
            RtType::ClosureType(i_t, o_t) =>
                if let RtType::ClosureType(..) = i_t.clone().deref() {
                    f.write_str(&*format!("({i_t:?}) -> {o_t:?}"))
                } else {
                    f.write_str(&*format!("{i_t:?} -> {o_t:?}"))
                },
            RtType::SumType(s) =>
                f.write_str(&*format!("SumType{s:?}")),
            RtType::ProdType(v) =>
                f.write_str(&*format!("ProdType{v:?}")),
        }
    }
}
