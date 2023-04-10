use std::collections::BTreeSet;

use crate::parser::r#type::Type;
use crate::type_checker::env::type_env::TypeEnv;

pub fn lift(
    type_env: &TypeEnv,
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
            .find_type(ref_name)
            .map(|t| lift(type_env, set, t))
            .unwrap_or(false),

        // .. | T | ..
        // where Base can be lifted to T
        Type::SumType(s) => s
            .iter()
            .rev()
            .any(|t| lift(type_env, set, t)),

        _ => false
    }
}
