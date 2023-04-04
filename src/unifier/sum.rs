use std::collections::BTreeSet;
use crate::parser::r#type::Type;

pub fn lift(
    type_env: &Vec<(String, Type)>,
    set: &BTreeSet<Type>,
    derive: &Type,
) -> bool {
    println!("Uplift SumType{:?} to {:?}", set, derive);

    match derive {
        // Derive is superset of Base
        Type::SumType(s)
        if s.is_superset(set) => true,

        // type Derive = T
        // where Base can be lifted to T
        Type::TypeEnvRef(a)
        => type_env
            .iter()
            .rev()
            .find(|(n, t)| n == a && lift(type_env, set, t))
            .is_some(),

        // type Derive = .. | T | ..
        // where Base can be lifted to T
        Type::SumType(s)
        => s
            .iter()
            .rev()
            .any(|t| lift(type_env, set, t)),

        _ => false
    }
}
