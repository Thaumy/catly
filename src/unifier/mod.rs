use closure::lift as lift_closure;
use env_ref::lift as lift_env_ref;
use prod::lift as lift_prod;
use sum::lift as lift_sum;

use crate::parser::infra::option::AnyExt;
use crate::parser::r#type::Type;

mod closure;
mod env_ref;
mod sum;
mod prod;

pub fn lift(type_env: &Vec<(String, Type)>, l: &Type, r: &Type) -> bool {
    match l {
        Type::TypeEnvRef(n)
        if lift_env_ref(type_env, n, r) => true,
        Type::ClosureType(i, o)
        if lift_closure(type_env, i, o, r) => true,
        Type::SumType(s)
        if lift_sum(type_env, s, r) => true,
        Type::ProdType(v)
        if lift_prod(type_env, v, r) => true,
        _ => false
    }
}

// TODO: 检查 env 中 Type 的合法性
pub fn unify(type_env: &Vec<(String, Type)>, l: &Type, r: &Type) -> Option<Type> {
    match true {
        _ if lift(type_env, l, r) => r.clone().some(),
        _ if lift(type_env, r, l) => l.clone().some(),
        _ => None
    }
}

#[cfg(test)]
mod test;
