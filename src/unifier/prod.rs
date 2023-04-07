use crate::parser::r#type::Type;
use crate::type_checker::get_type::r#type::TypeEnv;

pub fn lift(
    type_env: &TypeEnv,
    vec: &Vec<(String, Type)>,
    derive: &Type
) -> bool {
    println!("Uplift ProdType{:?} to {:?}", vec, derive);

    match derive {
        // Base
        Type::ProdType(v) => v == vec,

        // T
        // where Base can be lifted to T
        Type::TypeEnvRef(ref_name) => type_env
            .iter()
            .rev()
            .find(|(n, t)| n == ref_name && lift(type_env, vec, t))
            .is_some(),

        // .. | T | ..
        // where Base can be lifted to T
        Type::SumType(s) => s
            .iter()
            .rev()
            .any(|t| lift(type_env, vec, t)),

        _ => false
    }
}
