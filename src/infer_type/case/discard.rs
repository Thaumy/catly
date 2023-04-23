use crate::env::r#type::type_env::TypeEnv;
use crate::infer_type::r#fn::has_type;
use crate::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer_type::r#type::require_info::RequireInfo;
use crate::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::alias::MaybeType;

pub fn case(
    type_env: &TypeEnv,
    expect_type: &MaybeType
) -> InferTypeRet {
    match expect_type {
        Some(expect_type) =>
            if type_env.is_type_valid(expect_type) {
                has_type(expect_type.clone())
            } else {
                TypeMissMatch::of(&format!(
                    "{expect_type:?} not found in type env"
                ))
                .into()
            },
        // Discard 值必须具备类型信息
        None => RequireInfo::of("_", EnvRefConstraint::empty()).into()
    }
}