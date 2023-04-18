use crate::env::r#type::type_env::TypeEnv;
use crate::parser::r#type::r#type::Type;
use crate::unify::lift;

pub fn lift_prod(
    type_env: &TypeEnv,
    prod_vec: &Vec<(String, Type)>,
    derive: &Type
) -> Option<Type> {
    match derive {
        // Base
        Type::ProdType(v) => (v == prod_vec).then(|| derive.clone()),

        // T
        // where Base can be lifted to T
        Type::NamelyType(type_name) => type_env
            .find_type(type_name)
            .and_then(|t| {
                lift(type_env, &Type::ProdType(prod_vec.clone()), t)
                    .map(|_| derive.clone())
            }),

        // .. | T | ..
        // where Base can be lifted to T
        Type::SumType(s) => s
            .iter()
            .any(|t| {
                lift(type_env, &Type::ProdType(prod_vec.clone()), t)
                    .is_some()
            })
            .then(|| derive.clone()),

        _ => None
    }
}
