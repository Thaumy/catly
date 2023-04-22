use std::collections::BTreeSet;

use crate::env::r#type::type_env::TypeEnv;
use crate::infra::option::AnyExt;
use crate::parser::r#type::r#type::Type;

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
                Type::SumType(sum_set.clone())
                    .lift_to(type_env, t)
                    .map(|_| derive.clone())
            }),

        // .. | T | ..
        // where Base can be lifted to T
        Type::SumType(s) => s
            .iter()
            .any(|t| {
                Type::SumType(sum_set.clone())
                    .lift_to(type_env, t)
                    .is_some()
            })
            .then(|| derive.clone()),

        _ => None
    }
}
