use crate::infer::env::type_env::TypeEnv;
use crate::parser::r#type::r#type::Type;

pub fn lift_prod(
    type_env: &TypeEnv,
    prod_vec: &Vec<(String, Type)>,
    derive: &Type
) -> Option<Type> {
    if derive.is_primitive() {
        return None;
    }

    match derive {
        // Base
        Type::ProdType(v) => (v == prod_vec).then(|| derive.clone()),

        // T
        // where Base can be lifted to T
        Type::NamelyType(type_name) => type_env
            .find_type(type_name.as_str())
            .and_then(|t| {
                Type::ProdType(prod_vec.clone())
                    .lift_to(type_env, &t)
                    .map(|_| derive.clone())
            }),

        // .. | Base | ..
        Type::SumType(s) => s
            .iter()
            .any(|t| &Type::ProdType(prod_vec.clone()) == t)
            .then(|| derive.clone()),

        // 与 int case 同理
        // // .. | T | ..
        // // where Base can be lifted to T
        // Type::SumType(s) => s
        //     .iter()
        //     .any(|t| {
        //         Type::ProdType(prod_vec.clone())
        //             .can_lift_to(type_env, t)
        //     })
        //     .then(|| derive.clone()),
        _ => None
    }
}
