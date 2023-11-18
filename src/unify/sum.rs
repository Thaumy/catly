use std::collections::BTreeSet;

use crate::infer::env::TypeEnv;
use crate::infra::WrapOption;
use crate::parser::r#type::Type;

pub fn lift_sum(
    type_env: &TypeEnv,
    sum_set: &BTreeSet<Type>,
    derive: &Type
) -> Option<Type> {
    if derive.is_primitive() {
        return None;
    }

    match derive {
        // Superset of Base
        Type::SumType(s) if s.is_superset(sum_set) =>
            derive.clone().wrap_some(),

        // T
        // where Base can be lifted to T
        Type::NamelyType(type_name) => type_env
            .find_type(type_name.as_str())
            .and_then(|t| {
                Type::SumType(sum_set.clone())
                    .lift_to(type_env, &t)
                    .map(|_| derive.clone())
            }),

        // .. | Base | ..
        Type::SumType(s) => s
            .iter()
            .any(|t| &Type::SumType(sum_set.clone()) == t)
            .then(|| derive.clone()),

        // 与 int case 同理
        // // .. | T | ..
        // // where Base can be lifted to T
        // Type::SumType(s) => s
        //     .iter()
        //     .any(|t| {
        //         Type::SumType(sum_set.clone())
        //             .can_lift_to(type_env, t)
        //     })
        //     .then(|| derive.clone()),
        _ => None
    }
}
