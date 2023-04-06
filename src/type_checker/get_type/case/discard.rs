use crate::infra::alias::MaybeType;
use crate::type_checker::get_type::r#type::{GetTypeReturn, TypeEnv};
use crate::unifier::lift;
use crate::{discard_type, has_type, type_miss_match};

pub fn case(type_env: &TypeEnv, t: &MaybeType) -> GetTypeReturn {
    match t {
        Some(t) => match lift(type_env, &discard_type!(), t) {
            Some(t) => has_type!(t),
            _ => type_miss_match!()
        },
        // TODO: 考虑此类型的合理性
        _ => has_type!(discard_type!())
    }
}
