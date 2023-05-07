use crate::eval::eval_expr::EvalRet;
use crate::eval::r#type::expr::primitive_op::PrimitiveOp;
use crate::eval::r#type::expr::Expr;
use crate::eval::r#type::r#type::Type;
use crate::infra::result::ResultAnyExt;

#[cfg(test)]
mod test;

pub fn case_primitive_op(
    type_annot: &Type,
    op: &Box<PrimitiveOp>
) -> EvalRet {
    Expr::PrimitiveOp(type_annot.clone(), op.clone(), None).ok()
}
