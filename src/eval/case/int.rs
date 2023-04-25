use crate::eval::r#type::expr::Expr;
use crate::eval::r#type::r#type::Type;
use crate::eval::EvalRet;
use crate::infra::result::AnyExt;

pub fn case_int(type_annot: Type, int_value: i64) -> EvalRet {
    Expr::Int(type_annot, int_value).ok()
}
