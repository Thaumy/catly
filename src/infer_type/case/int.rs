use crate::env::r#type::type_env::TypeEnv;
use crate::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::int_type;
use crate::parser::r#type::r#type::MaybeType;

pub fn case(
    type_env: &TypeEnv,
    expect_type: &MaybeType
) -> InferTypeRet {
    InferTypeRet::from_auto_lift(
        type_env,
        &int_type!(),
        expect_type,
        None
    )
}
