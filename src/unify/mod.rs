use std::ops::Deref;

use closure::lift_closure;
use namely::lift_namely;
use prod::lift_prod;
use sum::lift_sum;

use crate::infer::env::type_env::TypeEnv;
use crate::parser::r#type::Type;

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
    #[cfg(feature = "unify_log")]
    {
        let log = format!(
            "{:8}{:>10} │ {from:?} to {to:?}",
            "[unify]", "Lift"
        );
        println!("{log}");
    }

    if let Type::NamelyType(n) = from
        && !type_env.exist_ref(n.as_str()) { return None; }
    if let Type::NamelyType(n) = to
        && !type_env.exist_ref(n.as_str()) { return None; }

    let result = match from {
        Type::NamelyType(n) => lift_namely(type_env, n.as_str(), to),
        Type::ClosureType(i, o) =>
            lift_closure(type_env, i.deref(), o.deref(), to),
        Type::SumType(s) => lift_sum(type_env, s, to),
        Type::ProdType(v) => lift_prod(type_env, v, to),

        // 不允许提升任何不完整类型, 它们仅能被用作推导提示
        _ => None
    };

    result
}
