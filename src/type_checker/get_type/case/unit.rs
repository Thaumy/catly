use crate::infra::alias::MaybeType;
use crate::type_checker::get_type::r#type::{GetTypeReturn, TypeEnv};
use crate::unifier::can_lift;
use crate::{has_type, unit_type};

pub fn case(type_env: &TypeEnv, t: &MaybeType) -> GetTypeReturn {
    match t {
        Some(t) if can_lift(type_env, &unit_type!(), &t) =>
            has_type!(t.clone()),
        _ => has_type!(unit_type!())
    }
}
