#[cfg(test)]
mod test;

use crate::infer::env::TypeEnv;
use crate::infer::infer_type::EnvRefConstraint;
use crate::infer::infer_type::InferTypeRet;
use crate::infer::infer_type::ReqInfo;
use crate::infer::infer_type::TypeMissMatch;
use crate::infra::WrapOption;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::OptType;

pub fn case(
    type_env: &TypeEnv,
    expect_type: &OptType
) -> InferTypeRet {
    match expect_type {
        Some(expect_type) =>
            if type_env.is_type_valid(expect_type) {
                InferTypeRet::has_type(Expr::Discard(
                    expect_type
                        .clone()
                        .wrap_some()
                ))
            } else {
                TypeMissMatch::of(format!(
                    "{expect_type:?} not found in type env"
                ))
                .into()
            },
        // Discard 值必须具备类型信息
        None => ReqInfo::of("_", EnvRefConstraint::empty()).into()
    }
}
