use crate::env::type_env::TypeEnv;
use crate::infra::alias::MaybeType;
use crate::type_checker::get_type::r#fn::lift_or_miss_match;
use crate::type_checker::get_type::r#type::GetTypeReturn;
use crate::{has_type, unit_type};

pub fn case(
    type_env: &TypeEnv,
    expect_type: &MaybeType
) -> GetTypeReturn {
    match expect_type {
        Some(expect_type) =>
            lift_or_miss_match(type_env, &unit_type!(), &expect_type),
        _ => has_type!(unit_type!())
    }
}
