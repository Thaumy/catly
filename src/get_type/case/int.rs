use crate::env::r#type::type_env::TypeEnv;
use crate::get_type::r#fn::lift_or_miss_match;
use crate::get_type::r#type::GetTypeReturn;
use crate::infra::alias::MaybeType;
use crate::{has_type, int_type};

pub fn case(
    type_env: &TypeEnv,
    expect_type: &MaybeType
) -> GetTypeReturn {
    match expect_type {
        Some(expect_type) =>
            lift_or_miss_match(type_env, &int_type!(), &expect_type),
        None => has_type!(int_type!())
    }
}
