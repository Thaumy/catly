use crate::env::r#type::type_env::TypeEnv;
use crate::get_type::r#type::type_miss_match::TypeMissMatch;
use crate::get_type::r#type::GetTypeReturn;
use crate::infra::alias::MaybeType;
use crate::{has_type, require_info};

pub fn case(
    type_env: &TypeEnv,
    expect_type: &MaybeType
) -> GetTypeReturn {
    match expect_type {
        Some(expect_type) =>
            if type_env.is_type_valid(expect_type) {
                has_type!(expect_type.clone())
            } else {
                TypeMissMatch::of(&format!(
                    "{expect_type:?} not found in type env"
                ))
                .into()
            },
        // Discard 值必须具备类型信息
        None => require_info!("_".to_string())
    }
}
