#[cfg(test)]
mod test;

use crate::infer::env::int_type;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::InferTypeRet;
use crate::infra::option::WrapOption;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::OptType;

pub fn case(
    type_env: &TypeEnv,
    expect_type: &OptType,
    i: &i64
) -> InferTypeRet {
    InferTypeRet::from_auto_lift(
        type_env,
        &int_type!(),
        expect_type,
        None,
        |t| Expr::Int(t.wrap_some(), *i)
    )
}
