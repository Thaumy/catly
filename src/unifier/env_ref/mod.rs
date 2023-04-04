use int::lift as lift_int;
use unit::lift as lift_unit;
use discard::lift as lift_discard;

use crate::parser::r#type::Type;

mod int;
mod unit;
mod discard;

pub fn lift(
    type_env: &Vec<(String, Type)>,
    base: &str,
    derive: &Type,
) -> bool {
    println!("Uplift {:?} to {:?}", base, derive);

    match base {
        "Int" => lift_int(type_env, derive),
        "Unit" => lift_unit(type_env, derive),
        "Discard" => lift_discard(type_env, derive),

        _ => match derive {
            // Derive is Base
            Type::TypeEnvRef(n)
            if n == base => true,

            // type Derive = .. | Base | ..
            Type::SumType(s)
            => s
                .iter()
                .rev()
                .any(|t| match t {
                    Type::TypeEnvRef(n) => n == base,
                    _ => false
                }),

            _ => false
        }
    }
}
