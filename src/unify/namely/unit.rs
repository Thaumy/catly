use crate::infer::env::type_env::TypeEnv;
use crate::infra::option::AnyExt;
use crate::parser::r#type::r#type::Type;
use crate::unit_type;

pub fn lift_unit(type_env: &TypeEnv, derive: &Type) -> Option<Type> {
    match derive {
        // Base
        Type::NamelyType(type_name) if type_name == "Unit" =>
            derive.clone().some(),

        // Int
        Type::NamelyType(type_name) if type_name == "Int" => None,

        // T
        // where Base can be lifted to T
        Type::NamelyType(type_name) => type_env
            .find_type(type_name.as_str())
            .and_then(|type_base| {
                unit_type!()
                    .lift_to(type_env, &type_base)
                    .map(|_| derive.clone())
            }),

        // .. | Unit | ..
        Type::SumType(s) => s
            .iter()
            .any(|t| match t {
                Type::NamelyType(n) => n == "Unit",
                _ => false
            })
            .then(|| derive.clone()),

        // // 与 int case 同理
        // // .. | T | ..
        // // where Base can be lifted to T
        // Type::SumType(s) => s
        //     .iter()
        //     .any(|t| {
        //         unit_type!()
        //             .lift_to(type_env, t)
        //             .is_some()
        //     })
        //     .then(|| derive.clone()),
        _ => None
    }
}
