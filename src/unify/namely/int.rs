use crate::infer::env::int_type;
use crate::infer::env::type_env::TypeEnv;
use crate::infra::WrapOption;
use crate::parser::r#type::Type;

pub fn lift_int(type_env: &TypeEnv, derive: &Type) -> Option<Type> {
    match derive {
        // Base
        Type::NamelyType(type_name) if type_name == "Int" =>
            derive.clone().wrap_some(),

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

        // .. | Int | ..
        Type::SumType(s) => s
            .iter()
            .any(|t| match t {
                Type::NamelyType(n) => n == "Int",
                _ => false
            })
            .then(|| derive.clone()),

        // // 基本类型只能逐步提升至目标类型
        // // 不允许下列提升过程, 因为可能导致运行时类型擦除
        // // .. | T | ..
        // // where Base can be lifted to T
        // Type::SumType(s) => s
        //     .iter()
        //     .any(|t| {
        //         int_type!()
        //             .lift_to(type_env, t)
        //             .is_some()
        //     })
        //     .then(|| derive.clone()),
        _ => None
    }
}
