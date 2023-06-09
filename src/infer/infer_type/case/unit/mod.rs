#[cfg(test)]
mod test;

use crate::infer::env::r#macro::unit_type;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infra::option::OptionAnyExt;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::OptType;

pub fn case(
    type_env: &TypeEnv,
    expect_type: &OptType
) -> InferTypeRet {
    InferTypeRet::from_auto_lift(
        type_env,
        &unit_type!(),
        expect_type,
        None,
        |t| Expr::Unit(t.some())
    )
}
