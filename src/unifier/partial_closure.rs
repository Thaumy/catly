use crate::env::type_env::TypeEnv;
use crate::parser::r#type::r#type::Type;
use crate::unifier::can_lift;

pub fn lift(type_env: &TypeEnv, i_t: &Type, derive: &Type) -> bool {
    match derive {
        // Base
        Type::PartialClosureType(d_i_t) =>
            can_lift(type_env, i_t, d_i_t),

        // ClosureType
        Type::ClosureType(d_i_t, _) => can_lift(type_env, i_t, d_i_t),

        // T
        // where Base can be lifted to T
        Type::NamelyType(ref_name) => type_env
            .find_type(ref_name)
            .map(|t| lift(type_env, i_t, t))
            .unwrap_or(false),

        // .. | T | ..
        // where Base can be lifted to T
        Type::SumType(s) => s
            .iter()
            .rev()
            .any(|t| lift(type_env, i_t, t)),

        _ => false
    }
}
