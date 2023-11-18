#[cfg(test)]
mod test;

use crate::infer::env::unit_type;
use crate::infer::env::TypeEnv;
use crate::infer::infer_type::InferTypeRet;
use crate::infra::WrapOption;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::OptType;

pub fn case(
    type_env: &TypeEnv,
    expect_type: &OptType
) -> InferTypeRet {
    InferTypeRet::from_auto_lift(
        type_env,
        &unit_type!(),
        expect_type,
        None,
        |t| Expr::Unit(t.wrap_some())
    )
}
