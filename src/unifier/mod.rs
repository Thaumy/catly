use std::ops::Deref;

use closure::lift as lift_closure;
use namely::lift as lift_namely;
use partial_closure::lift as lift_partial_closure;
use prod::lift as lift_prod;
use sum::lift as lift_sum;

use crate::env::type_env::TypeEnv;
use crate::infra::option::AnyExt;
use crate::infra::r#box::Ext;
use crate::parser::r#type::r#type::Type;

mod closure;
mod namely;
mod partial_closure;
mod prod;
mod sum;
#[cfg(test)]
mod test;

// TODO: 缓存以加快 lift 速度
pub fn can_lift(type_env: &TypeEnv, from: &Type, to: &Type) -> bool {
    println!("Lift <{from:?}> to <{to:?}>");

    if let Type::NamelyType(n) = from
        && !type_env.exist_ref(n) { return false; }
    if let Type::NamelyType(n) = to
        && !type_env.exist_ref(n) { return false; }

    match from {
        Type::NamelyType(n) if lift_namely(type_env, n, to) => true,
        Type::ClosureType(i, o)
            if lift_closure(type_env, i.deref(), o.deref(), to) =>
            true,
        Type::SumType(s) if lift_sum(type_env, s, to) => true,
        Type::ProdType(v) if lift_prod(type_env, v, to) => true,
        // Partial types
        Type::PartialClosureType(i)
            if lift_partial_closure(type_env, i, to) =>
            true,

        _ => false
    }
}

pub fn lift(
    type_env: &TypeEnv,
    from: &Type,
    to: &Type
) -> Option<Type> {
    // HACK
    // 特例, 不允许 ClosureType 退化为 PartialClosureType
    match (from, to) {
        (Type::ClosureType(..), Type::PartialClosureType(..)) =>
            return can_lift(type_env, to, from)
                .then_some(from.clone()),
        (
            Type::ClosureType(from_i_t, from_o_t),
            Type::ClosureType(to_i_t, to_o_t)
        ) => {
            let i_t = lift(type_env, from_i_t, to_i_t)?;
            let o_t = lift(type_env, from_o_t, to_o_t)?;
            return Type::ClosureType(i_t.boxed(), o_t.boxed())
                .some();
        }
        _ => {}
    }

    can_lift(type_env, from, to).then_some(to.clone())
}

pub fn unify(type_env: &TypeEnv, l: &Type, r: &Type) -> Option<Type> {
    match true {
        _ if can_lift(type_env, l, r) => r.clone().some(),
        _ if can_lift(type_env, r, l) => l.clone().some(),
        _ => None
    }
}
