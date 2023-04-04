use std::collections::BTreeSet;

use crate::parser::r#type::Type;

pub fn lift(
    type_env: &Vec<(String, Type)>,
    set: &BTreeSet<Type>,
    derive: &Type
) -> bool {
    println!("Uplift SumType{:?} to {:?}", set, derive);

    match derive {
        // Superset of Base
        Type::SumType(s) if s.is_superset(set) => true,

        // T
        // where Base can be lifted to T
        Type::TypeEnvRef(ref_name) => type_env
            .iter()
            .rev()
            .find(|(n, t)| n == ref_name && lift(type_env, set, t))
            .is_some(),

        // .. | T | ..
        // where Base can be lifted to T
        Type::SumType(s) => s
            .iter()
            .rev()
            .any(|t| lift(type_env, set, t)),

        _ => false
    }
}
