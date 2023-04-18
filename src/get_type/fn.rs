use crate::env::r#type::type_env::TypeEnv;
use crate::get_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::get_type::r#type::GetTypeReturn;
use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::parser::r#type::r#type::Type;
use crate::unify::{lift, lift_or_left};
use crate::{has_type, require_constraint, type_miss_match};

pub fn with_constraint_lift_or_left(
    constraint: EnvRefConstraint,
    type_env: &TypeEnv,
    base: &Type,
    derive: &MaybeType
) -> GetTypeReturn {
    match lift_or_left(type_env, base, derive) {
        // 按需传播
        Some(r#type) =>
            require_constraint_or_type(constraint, r#type),
        None => type_miss_match!(format!("{base:?} <> {derive:?}"))
    }
}

pub fn lift_or_miss_match(
    type_env: &TypeEnv,
    from: &Type,
    to: &Type
) -> GetTypeReturn {
    match lift(type_env, from, to) {
        Some(t) => has_type!(t),
        None => type_miss_match!(format!("{from:?} <> {from:?}"))
    }
}

pub fn with_constraint_lift_or_miss_match(
    constraint: EnvRefConstraint,
    type_env: &TypeEnv,
    from: &Type,
    to: &Type
) -> GetTypeReturn {
    match lift(type_env, from, to) {
        Some(r#type) =>
            require_constraint_or_type(constraint, r#type),
        None => type_miss_match!(format!("{from:?} <> {from:?}"))
    }
}

pub fn require_constraint_or_type(
    constraint: EnvRefConstraint,
    r#type: Type
) -> GetTypeReturn {
    if constraint.is_empty() {
        has_type!(r#type)
    } else {
        require_constraint!(r#type, constraint)
    }
}

pub fn destruct_namely_type(
    type_env: &TypeEnv,
    r#type: &Type
) -> Option<Type> {
    match r#type {
        Type::NamelyType(type_name) => {
            let base_type = type_env.find_type(type_name)?;
            destruct_namely_type(type_env, base_type)
        }
        x => x.clone().some()
    }
}
