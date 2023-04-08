use crate::infra::alias::MaybeType;
use crate::type_checker::get_type::r#type::{GetTypeReturn, TypeEnv};
use crate::unifier::can_lift;
use crate::{has_type, unit_type};

pub fn case(
    type_env: &TypeEnv,
    expect_type: &MaybeType
) -> GetTypeReturn {
    match expect_type {
        Some(expect_type)
            if can_lift(type_env, &unit_type!(), &expect_type) =>
            has_type!(expect_type.clone()),
        _ => has_type!(unit_type!())
    }
}
