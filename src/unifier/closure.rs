use crate::env::type_env::TypeEnv;
use crate::parser::r#type::Type;
use crate::unifier::can_lift;

pub fn lift(
    type_env: &TypeEnv,
    i_t: &Type,
    o_t: &Type,
    derive: &Type
) -> bool {
    let is_success = match derive {
        // Base
        Type::ClosureType(d_i_t, d_o_t) =>
        // 此处是类型信息的补全关系, 而非针对泛型的类型相容性判断
            can_lift(type_env, i_t, d_i_t) &&
                can_lift(type_env, o_t, d_o_t),

        // T
        // where Base can be lifted to T
        Type::TypeEnvRef(ref_name) => type_env
            .find_type(ref_name)
            .map(|t| lift(type_env, i_t, o_t, t))
            .unwrap_or(false),

        // .. | T | ..
        // where Base can be lifted to T
        Type::SumType(s) => s
            .iter()
            .rev()
            .any(|t| lift(type_env, i_t, o_t, t)),

        _ => false
    };

    println!(
        "ClosureType lifter: Lift {:?} -> {:?} to {:?} # {:?}",
        i_t, o_t, derive, is_success
    );

    is_success
}
