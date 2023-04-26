use std::collections::BTreeSet;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

use crate::infer::env::type_env::TypeEnv;
use crate::unify::lift;

pub type MaybeType = Option<Type>;

pub type ProdField = (String, Type);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    NamelyType(String),
    ClosureType(Box<Type>, Box<Type>),
    SumType(BTreeSet<Type>),
    ProdType(Vec<ProdField>),
    // Input type only
    PartialClosureType(Box<Type>)
}

impl Type {
    pub fn lift_to(
        &self,
        type_env: &TypeEnv,
        to: &Type
    ) -> Option<Type> {
        lift(type_env, self, to)
    }

    pub fn can_lift_to(&self, type_env: &TypeEnv, to: &Type) -> bool {
        self.lift_to(type_env, to)
            .is_some()
    }

    // TODO: 考虑不完整类型
    // Lift l to r if r exist, then return lifting result
    // Return l if r not exist
    pub fn lift_to_or_left(
        &self,
        type_env: &TypeEnv,
        to: &MaybeType
    ) -> Option<Type> {
        match to {
            Some(to) => self.lift_to(type_env, to),
            _ => Some(self.clone())
        }
    }

    pub fn unify(
        &self,
        type_env: &TypeEnv,
        with: &Type
    ) -> Option<Type> {
        // unify 会优先尝试从 l 到 r 的提升, 因此将目标类型放在右侧会更有效率
        self.lift_to(type_env, with)
            .or_else(|| with.lift_to(type_env, self))
    }

    pub fn is_partial(&self) -> bool {
        match self {
            Type::NamelyType(_) => false,
            Type::ClosureType(i_t, o_t) =>
                i_t.is_partial() || o_t.is_partial(),
            Type::SumType(sum_set) => sum_set
                .iter()
                .any(|t| t.is_partial()),
            Type::ProdType(prod_vec) => prod_vec
                .iter()
                .any(|(_, t)| t.is_partial()),
            Type::PartialClosureType(..) => true
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            Type::NamelyType(type_name) =>
                type_name == "Int" || type_name == "Unit",
            _ => false
        }
    }

    pub fn eq_or_more_specific_than(&self, other: &Type) -> bool {
        if self == other {
            return true;
        }

        match (self, other) {
            (
                Type::ClosureType(a, _),
                Type::PartialClosureType(b)
            ) => a.eq_or_more_specific_than(b),
            (Type::ClosureType(a, c), Type::ClosureType(b, d)) =>
                a.eq_or_more_specific_than(b) &&
                    c.eq_or_more_specific_than(d),
            // 目前只有 ClosureType 存在不完整类型
            _ => false
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::NamelyType(t_n) =>
                f.write_str(&*format!("'{t_n}'")),

            Type::ClosureType(i_t, o_t) =>
                if let Type::ClosureType(..) |
                Type::PartialClosureType(..) = i_t.clone().deref()
                {
                    f.write_str(&*format!("({i_t:?}) -> {o_t:?}"))
                } else {
                    f.write_str(&*format!("{i_t:?} -> {o_t:?}"))
                },

            Type::SumType(s_s) =>
                f.write_str(&*format!("SumType{s_s:?}")),

            Type::ProdType(p_v) =>
                f.write_str(&*format!("ProdType{p_v:?}")),

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
