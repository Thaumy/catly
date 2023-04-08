use crate::infra::alias::MaybeType;
use crate::parser::r#type::Type;
use crate::type_checker::get_type::r#type::{GetTypeReturn, TypeEnv};
use crate::unifier::ref_exist;
use crate::{has_type, require_info, type_miss_match};

pub fn case(
    type_env: &TypeEnv,
    expect_type: &MaybeType
) -> GetTypeReturn {
    match expect_type {
        Some(expect_type) => {
            if let Type::TypeEnvRef(ref_name) = expect_type && !ref_exist(type_env, ref_name) {
                return type_miss_match!();
            }
            has_type!(expect_type.clone())
        }
        // Discard 值必须具备类型信息
        _ => require_info!("_".to_string())
    }
}
