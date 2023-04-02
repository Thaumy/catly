use crate::parser::r#type::Type;

pub fn lift(
    env: &Vec<(String, Type)>,
    i_t: &Type,
    o_t: &Type,
    derive: &Type,
) -> bool {
    println!("Uplift {:?} -> {:?} to {:?}", i_t, o_t, derive);

    match derive {
        // Derive is Base
        Type::ClosureType(x, y)
        => &**x == i_t && &**y == o_t,

        // type Derive = A
        // where A can be lifted to I -> O
        Type::TypeEnvRef(a)
        => env
            .iter()
            .rev()
            .find(|(n, t)| n == a && lift(env, i_t, o_t, t))
            .is_some(),

        // type Derive = .. | A | ..
        // where A can be lifted to I -> O
        Type::SumType(set)
        => set
            .iter()
            .any(|t| lift(env, i_t, o_t, t)),

        _ => false
    }
}
