use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::parser::r#type::r#type::OptType;
use crate::unit_type;

pub fn case(
    type_env: &TypeEnv,
    expect_type: &OptType
) -> InferTypeRet {
    InferTypeRet::from_auto_lift(
        type_env,
        &unit_type!(),
        expect_type,
        None
    )
}
