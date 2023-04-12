use std::collections::BTreeSet;

use crate::env::type_env::TypeEnv;
use crate::parser::r#type::Type;

pub fn lift(
    type_env: &TypeEnv,
    set: &BTreeSet<Type>,
    derive: &Type
) -> bool {
    let is_success = match derive {
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
    };

    println!(
        "SumType lifter: Lift {:?} to {:?} # {:?}",
        set, derive, is_success
    );

    is_success
}
