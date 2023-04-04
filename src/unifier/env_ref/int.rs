use crate::parser::r#type::Type;

pub fn lift(env: &Vec<(String, Type)>, derive: &Type) -> bool {
    println!("Uplift Int to {:?}", derive);

    match derive {
        // Derive is Base
        Type::TypeEnvRef(n) if n == "Int" => true,

        // type Derive = T
        // where Base can be lifted to T
        Type::TypeEnvRef(a) => env
            .iter()
            .rev()
            .find(|(n, t)| n == a && lift(env, t))
            .is_some(),

        // type Derive = .. | T | ..
        // where Base can be lifted to T
        Type::SumType(s) => s.iter().any(|t| lift(env, t)),

        _ => false,
    }
}
