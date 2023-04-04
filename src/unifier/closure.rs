use crate::parser::r#type::Type;

pub fn lift(
    type_env: &Vec<(String, Type)>,
    i_t: &Type,
    o_t: &Type,
    derive: &Type
) -> bool {
    println!("Uplift {:?} -> {:?} to {:?}", i_t, o_t, derive);

    match derive {
        // Base
        Type::ClosureType(x, y) => &**x == i_t && &**y == o_t,

        // T
        // where Base can be lifted to T
        Type::TypeEnvRef(ref_name) => type_env
            .iter()
            .rev()
            .find(|(n, t)| {
                n == ref_name && lift(type_env, i_t, o_t, t)
            })
            .is_some(),

        // .. | T | ..
        // where Base can be lifted to T
        Type::SumType(s) => s
            .iter()
            .rev()
            .any(|t| lift(type_env, i_t, o_t, t)),

        _ => false
    }
}
