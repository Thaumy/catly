#[cfg(test)]
mod test;

use crate::eval::eval_expr::EvalRet;
use crate::eval::r#type::expr::Expr;
use crate::eval::r#type::r#type::Type;
use crate::infra::result::AnyExt;

pub fn case_unit(type_annot: Type) -> EvalRet {
    Expr::Unit(type_annot).ok()
}
