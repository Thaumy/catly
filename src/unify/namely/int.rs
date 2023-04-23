use crate::env::r#type::type_env::TypeEnv;
use crate::infra::option::AnyExt;
use crate::int_type;
use crate::parser::r#type::r#type::Type;

pub fn lift_int(type_env: &TypeEnv, derive: &Type) -> Option<Type> {
    match derive {
        // Base
        Type::NamelyType(type_name) if type_name == "Int" =>
            derive.clone().some(),

        // Unit
        Type::NamelyType(type_name) if type_name == "Unit" => None,

        // T
        // where Base can be lifted to T
        Type::NamelyType(type_name) => type_env
            .find_type(type_name.as_str())
            .and_then(|type_base| {
                int_type!()
                    .lift_to(type_env, &type_base)
                    .map(|_| derive.clone())
            }),

        // .. | T | ..
        // where Base can be lifted to T
        Type::SumType(s) => s
            .iter()
            .any(|t| {
                int_type!()
                    .lift_to(type_env, t)
                    .is_some()
            })
            .then(|| derive.clone()),

        _ => None
    }
}
