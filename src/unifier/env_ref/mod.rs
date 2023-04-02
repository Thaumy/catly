use int::lift as lift_int;
use unit::lift as lift_unit;

use crate::parser::r#type::Type;

mod int;
mod unit;

pub fn lift(
    env: &Vec<(String, Type)>,
    base: &str,
    derive: &Type,
) -> bool {
    println!("Uplift {:?} to {:?}", base, derive);

    match base {
        "Int" => lift_int(env, derive),
        "Unit" => lift_unit(env, derive),
        _ => match derive {
            // Derive is Base
            Type::TypeEnvRef(n)
            if n == base => true,

            // type Derive = .. | Base | ..
            Type::SumType(s)
            => s
                .iter()
                .any(|t| match t {
                    Type::TypeEnvRef(n) => n == base,
                    _ => false
                }),

            _ => false
        }
    }
}
