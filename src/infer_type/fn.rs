use crate::env::r#type::type_env::TypeEnv;
use crate::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer_type::r#type::require_constraint::require_constraint;
use crate::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infer_type::r#type::GetTypeReturn;
use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::parser::r#type::r#type::Type;

pub fn with_constraint_lift_or_left(
    constraint: EnvRefConstraint,
    type_env: &TypeEnv,
    base: &Type,
    derive: &MaybeType
) -> GetTypeReturn {
    match base.lift_to_or_left(type_env, derive) {
        // 按需传播
        Some(r#type) => require_constraint(r#type, constraint),
        None => TypeMissMatch::of_type(base, &derive.clone().unwrap())
            .into()
    }
}

pub fn lift_or_miss_match(
    type_env: &TypeEnv,
    from: &Type,
    to: &Type
) -> GetTypeReturn {
    match from.lift_to(type_env, to) {
        Some(t) => has_type(t),
        None => TypeMissMatch::of_type(from, to).into()
    }
}

pub fn with_constraint_lift_or_miss_match(
    constraint: EnvRefConstraint,
    type_env: &TypeEnv,
    from: &Type,
    to: &Type
) -> GetTypeReturn {
    match from.lift_to(type_env, to) {
        Some(r#type) => require_constraint(r#type, constraint),
        None => TypeMissMatch::of_type(from, to).into()
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

pub fn has_type(r#type: Type) -> GetTypeReturn { Quad::L(r#type) }
