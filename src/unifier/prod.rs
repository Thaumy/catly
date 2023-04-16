use crate::env::type_env::TypeEnv;
use crate::parser::r#type::r#type::Type;

pub fn lift(
    type_env: &TypeEnv,
    vec: &Vec<(String, Type)>,
    derive: &Type
) -> bool {
    match derive {
        // Base
        Type::ProdType(v) => v == vec,

        // T
        // where Base can be lifted to T
        Type::NamelyType(ref_name) => type_env
            .find_type(ref_name)
            .map(|t| lift(type_env, vec, t))
            .unwrap_or(false),

        // .. | T | ..
        // where Base can be lifted to T
        Type::SumType(s) => s
            .iter()
            .rev()
            .any(|t| lift(type_env, vec, t)),

        _ => false
    }
}
