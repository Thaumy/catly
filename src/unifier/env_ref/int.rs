use crate::parser::r#type::Type;

pub fn lift(env: &Vec<(String, Type)>, derive: &Type) -> bool {
    println!("Uplift Int to {:?}", derive);

    match derive {
        // Base
        Type::TypeEnvRef(n) if n == "Int" => true,

        // T
        // where Base can be lifted to T
        Type::TypeEnvRef(ref_name) => env
            .iter()
            .rev()
            .find(|(n, t)| n == ref_name && lift(env, t))
            .is_some(),

        // .. | T | ..
        // where Base can be lifted to T
        Type::SumType(s) => s.iter().any(|t| lift(env, t)),

        _ => false
    }
}
