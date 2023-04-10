use crate::parser::r#type::Type;
use crate::type_checker::env::type_env::TypeEnv;

pub fn lift(env: &TypeEnv, derive: &Type) -> bool {
    println!("Uplift Int to {:?}", derive);

    match derive {
        // Base
        Type::TypeEnvRef(n) if n == "Int" => true,

        // T
        // where Base can be lifted to T
        Type::TypeEnvRef(ref_name) => env
            .find_type(ref_name)
            .map(|t| lift(env, t))
            .unwrap_or(false),

        // .. | T | ..
        // where Base can be lifted to T
        Type::SumType(s) => s.iter().any(|t| lift(env, t)),

        _ => false
    }
}
