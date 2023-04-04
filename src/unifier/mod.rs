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

pub fn lift(
    type_env: &Vec<(String, Type)>,
    l: &Type,
    r: &Type
) -> bool {
    if let Type::TypeEnvRef(n) = l
        && !ref_exist(type_env, n) { return false; }
    if let Type::TypeEnvRef(n) = r
        && !ref_exist(type_env, n) { return false; }

    match l {
        Type::TypeEnvRef(n) if lift_env_ref(type_env, n, r) => true,
        Type::ClosureType(i, o)
            if lift_closure(type_env, i, o, r) =>
            true,
        Type::SumType(s) if lift_sum(type_env, s, r) => true,
        Type::ProdType(v) if lift_prod(type_env, v, r) => true,
        _ => false
    }
}

pub fn unify(
    type_env: &Vec<(String, Type)>,
    l: &Type,
    r: &Type
) -> Option<Type> {
    match true {
        _ if lift(type_env, l, r) => r.clone().some(),
        _ if lift(type_env, r, l) => l.clone().some(),
        _ => None
    }
}

#[cfg(test)]
mod test;
