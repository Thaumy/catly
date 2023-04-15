use std::ops::Deref;

use closure::lift as lift_closure;
use namely::lift as lift_namely;
use prod::lift as lift_prod;
use sum::lift as lift_sum;

use crate::env::type_env::TypeEnv;
use crate::infra::option::AnyExt;
use crate::parser::r#type::Type;

mod closure;
mod namely;
mod prod;
mod sum;
#[cfg(test)]
mod test;

pub fn can_lift(type_env: &TypeEnv, from: &Type, to: &Type) -> bool {
    if let Type::NamelyType(n) = from
        && !type_env.exist_ref( n) { return false; }
    if let Type::NamelyType(n) = to
        && !type_env.exist_ref( n) { return false; }

    match from {
        Type::NamelyType(n) if lift_namely(type_env, n, to) => true,
        Type::ClosureType(i, o)
            if lift_closure(type_env, i.deref(), o.deref(), to) =>
            true,
        Type::SumType(s) if lift_sum(type_env, s, to) => true,
        Type::ProdType(v) if lift_prod(type_env, v, to) => true,
        _ => false
    }
}

pub fn lift(
    type_env: &TypeEnv,
    from: &Type,
    to: &Type
) -> Option<Type> {
    can_lift(type_env, from, to).then_some(to.clone())
}

pub fn unify(type_env: &TypeEnv, l: &Type, r: &Type) -> Option<Type> {
    match true {
        _ if can_lift(type_env, l, r) => r.clone().some(),
        _ if can_lift(type_env, r, l) => l.clone().some(),
        _ => None
    }
}
