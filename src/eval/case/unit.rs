use crate::eval::r#type::expr::Expr;
use crate::eval::r#type::r#type::Type;
use crate::eval::EvalRet;
use crate::infra::result::AnyExt;

pub fn case_unit(type_annot: Type) -> EvalRet {
    Expr::Unit(type_annot).ok()
}
