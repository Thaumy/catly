use crate::eval::r#type::eval_err::EvalErr;
use crate::eval::r#type::r#type::Type;
use crate::eval::EvalRet;
use crate::infra::result::AnyExt;

pub fn case_discard(type_annot: &Type) -> EvalRet {
    EvalErr::of(format!("Trying to eval _:{type_annot:?}")).err()
}
