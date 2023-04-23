use int::lift_int;
use unit::lift_unit;

use crate::env::r#type::type_env::TypeEnv;
use crate::infra::option::AnyExt;
use crate::parser::r#type::r#type::Type;

mod int;
mod unit;

pub fn lift_namely(
    type_env: &TypeEnv,
    base_type_name: &str,
    derive: &Type
) -> Option<Type> {
    match base_type_name {
        "Int" => lift_int(type_env, derive),
        "Unit" => lift_unit(type_env, derive),

        _ => match derive {
            // Int or Unit
            Type::NamelyType(type_name)
                if type_name == "Int" || type_name == "Unit" =>
                None,

            // Base
            Type::NamelyType(n) if n == base_type_name =>
                derive.clone().some(),

            // HACK:
            // 该实现允许将 Base 合一到基于 SumType 的 NamelyType, 例如：
            // type True = Int
            // type False = Int
            // type Bool = True | False
            // 将 True 和 Bool 合一是可行的, 这会产生 Bool
            Type::NamelyType(n) => match type_env.find_type(n) {
                Some(Type::SumType(s)) => s
                    .iter()
                    .any(|t| match t {
                        Type::NamelyType(n) => n == base_type_name,
                        _ => false
                    })
                    .then(|| derive.clone()),
                _ => None
            },

            // .. | Base | ..
            Type::SumType(s) => s
                .iter()
                .any(|t| match t {
                    Type::NamelyType(n) => n == base_type_name,
                    _ => false
                })
                .then(|| derive.clone()),

            _ => None
        }
    }
}
