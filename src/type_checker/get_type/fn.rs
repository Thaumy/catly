use crate::env::type_env::TypeEnv;
use crate::infra::alias::MaybeType;
use crate::parser::r#type::Type;
use crate::type_checker::get_type::r#type::{
    EnvRefConstraint,
    GetTypeReturn
};
use crate::unifier::lift;
use crate::{has_type, require_constraint, type_miss_match};

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

pub fn with_constraint_lift_or_left(
    constraint: EnvRefConstraint,
    type_env: &TypeEnv,
    base: &Type,
    derive: &MaybeType
) -> GetTypeReturn {
    match lift_or_left(type_env, base, derive) {
        Some(t) =>
        // 按需传播
            if constraint.is_empty() {
                has_type!(t)
            } else {
                require_constraint!(t, constraint)
            },
        None => type_miss_match!()
    }
}

pub fn destruct_type_env_ref(
    type_env: &TypeEnv,
    r#type: &Type
) -> Option<Type> {
    match r#type {
        Type::NamelyType(ref_name) => {
            let base_type = type_env.find_type(ref_name)?;
            destruct_type_env_ref(type_env, base_type)
        }
        x => Some(x.clone())
    }
}
