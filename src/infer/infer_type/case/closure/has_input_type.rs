use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infra::r#box::BoxAnyExt;
use crate::parser::r#type::r#type::OptType;
use crate::parser::r#type::r#type::Type;

pub fn has_input_type(
    type_env: &TypeEnv,
    expect_type: &OptType,
    output_expr_type: Type,
    input_type: Type
) -> InferTypeRet {
    let base = Type::ClosureType(
        input_type.clone().boxed(),
        output_expr_type.boxed()
    );

    InferTypeRet::from_auto_lift(type_env, &base, expect_type, None)
}
