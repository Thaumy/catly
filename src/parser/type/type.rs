use std::collections::BTreeSet;
use std::fmt::{Debug, Formatter};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    NamelyType(String),
    ClosureType(Box<Type>, Box<Type>),
    SumType(BTreeSet<Type>),
    ProdType(Vec<(String, Type)>),
    // Input type only
    PartialClosureType(Box<Type>)
}

/*#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IncompleteType {
    PartialClosureType(Box<Type>)
}
*/
impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::NamelyType(n) => f.write_str(&*format!("'{n}'")),
            Type::ClosureType(i_t, o_t) =>
                f.write_str(&*format!("{i_t:?} -> {o_t:?}")),
            Type::SumType(s) =>
                f.write_str(&*format!("SumType{s:?}")),
            Type::ProdType(v) =>
                f.write_str(&*format!("ProdType{v:?}")),
            // Input type only
            Type::PartialClosureType(i_t) =>
                f.write_str(&*format!("{i_t:?} -> ?")),
        }
    }
}
