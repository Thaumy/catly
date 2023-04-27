use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::int_type;
use crate::parser::r#type::r#type::OptType;

pub fn case(
    type_env: &TypeEnv,
    expect_type: &OptType
) -> InferTypeRet {
    InferTypeRet::from_auto_lift(
        type_env,
        &int_type!(),
        expect_type,
        None
    )
}
