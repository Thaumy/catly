use crate::env::type_env::TypeEnv;
use crate::infra::alias::MaybeType;
use crate::type_checker::get_type::r#type::GetTypeReturn;
use crate::unifier::can_lift;
use crate::{has_type, int_type, type_miss_match};

pub fn case(
    type_env: &TypeEnv,
    expect_type: &MaybeType
) -> GetTypeReturn {
    match expect_type {
        Some(expect_type) =>
            if can_lift(type_env, &int_type!(), &expect_type) {
                has_type!(expect_type.clone())
            } else {
                type_miss_match!()
            },
        _ => has_type!(int_type!())
    }
}
