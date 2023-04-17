use std::collections::BTreeSet;

use crate::env::type_env::TypeEnv;
use crate::infra::option::AnyExt;
use crate::parser::r#type::r#type::Type;
use crate::unify::lift;

pub fn lift_sum(
    type_env: &TypeEnv,
    sum_set: &BTreeSet<Type>,
    derive: &Type
) -> Option<Type> {
    match derive {
        // Superset of Base
        Type::SumType(s) if s.is_superset(sum_set) =>
            derive.clone().some(),

        // T
        // where Base can be lifted to T
        Type::NamelyType(type_name) => type_env
            .find_type(type_name)
            .and_then(|t| {
                lift(type_env, &Type::SumType(sum_set.clone()), t)
                    .map(|_| derive.clone())
            }),

        // .. | T | ..
        // where Base can be lifted to T
        Type::SumType(s) => s
            .iter()
            .any(|t| {
                lift(type_env, &Type::SumType(sum_set.clone()), t)
                    .is_some()
            })
            .then(|| derive.clone()),

        _ => None
    }
}
