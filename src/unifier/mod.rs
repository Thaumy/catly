use closure::lift as lift_closure;
use env_ref::lift as lift_env_ref;
use prod::lift as lift_prod;
use sum::lift as lift_sum;

use crate::infra::option::AnyExt;
use crate::parser::r#type::Type;
use crate::type_checker::env::type_env::TypeEnv;

mod closure;
mod env_ref;
mod prod;
mod sum;

pub fn can_lift(type_env: &TypeEnv, from: &Type, to: &Type) -> bool {
    if let Type::TypeEnvRef(n) = from
        && !type_env.exist_ref( n) { return false; }
    if let Type::TypeEnvRef(n) = to
        && !type_env.exist_ref( n) { return false; }

    match from {
        Type::TypeEnvRef(n) if lift_env_ref(type_env, n, to) => true,
        Type::ClosureType(i, o)
            if lift_closure(
                type_env,
                &i.clone().map(|x| *x),
                &o.clone().map(|x| *x),
                to
            ) =>
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

#[cfg(test)]
mod test;
