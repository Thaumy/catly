use crate::infra::alias::MaybeType;
use crate::parser::expr::Expr;
use crate::type_checker::get_type::r#type::GetTypeReturn;

pub fn case(
    expect_type: &MaybeType,
    lhs: &Expr,
    rhs: &Expr
) -> GetTypeReturn {
    todo!()
}
