use crate::env::type_env::TypeEnv;
use crate::parser::r#type::Type;

pub fn lift(type_env: &TypeEnv, derive: &Type) -> bool {
    println!("Lift {:?} to {:?}", "Unit", derive);

    match derive {
        // Base
        Type::TypeEnvRef(ref_name) if ref_name == "Unit" => true,

        // T
        // where Base can be lifted to T
        Type::TypeEnvRef(ref_name) => type_env
            .find_type(ref_name)
            .map(|t| lift(type_env, t))
            .unwrap_or(false),

        // .. | T | ..
        // where Base can be lifted to T
        Type::SumType(s) => s
            .iter()
            .any(|t| lift(type_env, t)),

        _ => false
    }
}
