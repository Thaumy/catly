use std::collections::BTreeSet;
use crate::parser::r#type::Type;

pub fn lift(
    env: &Vec<(String, Type)>,
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
        => env
            .iter()
            .rev()
            .find(|(n, t)| n == a && lift(env, set, t))
            .is_some(),

        // type Derive = .. | T | ..
        // where Base can be lifted to T
        Type::SumType(s)
        => s
            .iter()
            .any(|t| lift(env, set, t)),

        _ => false
    }
}
