use crate::parser::r#type::Type;

pub fn lift(
    env: &Vec<(String, Type)>,
    vec: &Vec<(String, Type)>,
    derive: &Type,
) -> bool {
    println!("Uplift ProductType{:?} to {:?}", vec, derive);

    match &derive {
        // Derive is Base
        Type::ProductType(v)
        => v == vec,

        // type Derive = T
        // where Base can be lifted to T
        Type::TypeEnvRef(a)
        => env
            .iter()
            .rev()
            .find(|(n, t)| n == a && lift(env, vec, t))
            .is_some(),

        // type Derive = .. | T | ..
        // where Base can be lifted to T
        Type::SumType(s)
        => s
            .iter()
            .any(|t| lift(env, vec, t)),

        _ => false
    }
}
