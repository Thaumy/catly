use crate::parser::r#type::Type;

pub fn lift(type_env: &Vec<(String, Type)>, vec: &Vec<(String, Type)>, derive: &Type) -> bool {
    println!("Uplift ProdType{:?} to {:?}", vec, derive);

    match derive {
        // Derive is Base
        Type::ProdType(v) => v == vec,

        // type Derive = T
        // where Base can be lifted to T
        Type::TypeEnvRef(a) => type_env
            .iter()
            .rev()
            .find(|(n, t)| n == a && lift(type_env, vec, t))
            .is_some(),

        // type Derive = .. | T | ..
        // where Base can be lifted to T
        Type::SumType(s) => s.iter().rev().any(|t| lift(type_env, vec, t)),

        _ => false,
    }
}
