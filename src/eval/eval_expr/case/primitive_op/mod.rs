use std::rc::Rc;

use crate::eval::eval_expr::EvalRet;
use crate::eval::Expr;
use crate::eval::PrimitiveOp;
use crate::eval::Type;
use crate::infra::WrapResult;

#[cfg(test)]
mod test;

pub fn case_primitive_op(
    type_annot: &Type,
    op: &Rc<PrimitiveOp>
) -> EvalRet {
    Expr::PrimitiveOp(type_annot.clone(), op.clone(), None).wrap_ok()
}
