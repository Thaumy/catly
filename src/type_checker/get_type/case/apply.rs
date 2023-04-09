use crate::infra::alias::MaybeType;
use crate::parser::expr::Expr;
use crate::type_checker::get_type::r#type::{
    ExprEnv,
    GetTypeReturn,
    TypeEnv
};

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &MaybeType,
    lhs: &Expr,
    rhs: &Expr
) -> GetTypeReturn {
    todo!()
}
