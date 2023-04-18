use std::ops::Deref;

use closure::lift_closure;
use namely::lift_namely;
use prod::lift_prod;
use sum::lift_sum;

use crate::env::r#type::type_env::TypeEnv;
use crate::infra::alias::MaybeType;
use crate::parser::r#type::r#type::Type;

mod closure;
mod namely;
mod prod;
mod sum;
#[cfg(test)]
mod test;

pub fn lift(
    type_env: &TypeEnv,
    from: &Type,
    to: &Type
) -> Option<Type> {
    println!("{:8}{:>10} │ {from:?} to {to:?}", "[unify]", "Lift");

    if let Type::NamelyType(n) = from
        && !type_env.exist_ref(n) { return None; }
    if let Type::NamelyType(n) = to
        && !type_env.exist_ref(n) { return None; }

    let result = match from {
        Type::NamelyType(n) => lift_namely(type_env, n, to),
        Type::ClosureType(i, o) =>
            lift_closure(type_env, i.deref(), o.deref(), to),
        Type::SumType(s) => lift_sum(type_env, s, to),
        Type::ProdType(v) => lift_prod(type_env, v, to),

        // 不允许提升任何不完全类型, 它们仅能被用作推导提示
        _ => None
    };

    result
}

pub fn can_lift(type_env: &TypeEnv, from: &Type, to: &Type) -> bool {
    lift(type_env, from, to).is_some()
}

// Lift l to r if r exist, then return lifting result
// Return l if r not exist
pub fn lift_or_left(
    type_env: &TypeEnv,
    l: &Type,
    r: &MaybeType
) -> Option<Type> {
    match r {
        Some(r) => lift(type_env, &l, &r),
        _ => Some(l.clone())
    }
}

pub fn unify(type_env: &TypeEnv, l: &Type, r: &Type) -> Option<Type> {
    // unify 会优先尝试从 l 到 r 的提升, 因此将目标类型放在右侧会更有效率
    lift(type_env, l, r).or_else(|| lift(type_env, r, l))
}
