use int::lift_int;
use unit::lift_unit;

use crate::infer::env::TypeEnv;
use crate::infra::WrapOption;
use crate::parser::r#type::Type;

mod int;
mod unit;

pub fn lift_namely<'s>(
    type_env: &TypeEnv,
    base_type_name: impl Into<&'s str>,
    derive: &Type
) -> Option<Type> {
    let base_type_name = base_type_name.into();
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
                derive.clone().wrap_some(),

            // HACK:
            // 该实现允许将 Base 合一到基于 SumType 的 NamelyType, 例如：
            // type True = Int
            // type False = Int
            // type Bool = True | False
            // 将 True 和 Bool 合一是可行的, 这会产生 Bool
            Type::NamelyType(n) => match type_env
                .find_type(n.as_str())
            {
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
