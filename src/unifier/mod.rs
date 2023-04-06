use closure::lift as lift_closure;
use env_ref::lift as lift_env_ref;
use prod::lift as lift_prod;
use sum::lift as lift_sum;

use crate::infra::option::AnyExt;
use crate::parser::r#type::Type;

mod closure;
mod env_ref;
mod prod;
mod sum;

fn ref_exist(type_env: &Vec<(String, Type)>, ref_name: &str) -> bool {
    let is_exist = match ref_name {
        "Int" | "Unit" | "Discard" => true,
        _ => type_env
            .iter()
            .any(|(n, _)| n == ref_name)
    };
    if !is_exist {
        println!("TypeEnvRef {:?} not exist in type env", ref_name);
    }
    is_exist
}

pub fn can_lift(
    type_env: &Vec<(String, Type)>,
    from: &Type,
    to: &Type
) -> bool {
    if let Type::TypeEnvRef(n) = from
        && !ref_exist(type_env, n) { return false; }
    if let Type::TypeEnvRef(n) = to
        && !ref_exist(type_env, n) { return false; }

    match from {
        Type::TypeEnvRef(n) if lift_env_ref(type_env, n, to) => true,
        Type::ClosureType(i, o)
            if lift_closure(type_env, i, o, to) =>
            true,
        Type::SumType(s) if lift_sum(type_env, s, to) => true,
        Type::ProdType(v) if lift_prod(type_env, v, to) => true,
        _ => false
    }
}

pub fn lift(
    type_env: &Vec<(String, Type)>,
    from: &Type,
    to: &Type
) -> Option<Type> {
    can_lift(type_env, from, to).then_some(to.clone())
}

pub fn unify(
    type_env: &Vec<(String, Type)>,
    l: &Type,
    r: &Type
) -> Option<Type> {
    match true {
        _ if can_lift(type_env, l, r) => r.clone().some(),
        _ if can_lift(type_env, r, l) => l.clone().some(),
        _ => None
    }
}

#[cfg(test)]
mod test;
