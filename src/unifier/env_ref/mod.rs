use discard::lift as lift_discard;
use int::lift as lift_int;
use unit::lift as lift_unit;

use crate::parser::r#type::Type;

mod discard;
mod int;
mod unit;

pub fn lift(
    type_env: &Vec<(String, Type)>,
    base: &str,
    derive: &Type
) -> bool {
    println!("Uplift {:?} to {:?}", base, derive);

    match base {
        "Int" => lift_int(type_env, derive),
        "Unit" => lift_unit(type_env, derive),
        "Discard" => lift_discard(derive),

        _ => match derive {
            // Base
            Type::TypeEnvRef(ref_name) if ref_name == base => true,

            /* HACK:
            该实现允许将 Base 合一到基于 SumType 的 TypeEnvRef, 例如：
            type True = Int
            type False = Int
            type Bool = True | False
            将 True 和 Bool 合一是可行的, 这会产生 Bool */
            // type Derive = .. | Base | ..
            Type::TypeEnvRef(ref_name) if let Some(t) = type_env
                .iter()
                .rev()
                .find(|(n, _)| n == ref_name)
                .map(|(_, t)| t)
                && let Type::SumType(s) = t
                && s.iter().rev().any(|t| match t {
                Type::TypeEnvRef(n) => n == base,
                _ => false,
            })
            => true,

            // .. | Base | ..
            Type::SumType(s) => s.iter().rev().any(|t| match t {
                Type::TypeEnvRef(n) => n == base,
                _ => false,
            }),
            _ => false,
        },
    }
}
