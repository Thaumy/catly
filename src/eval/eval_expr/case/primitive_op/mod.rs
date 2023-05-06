use crate::eval::env::expr_env::ExprEnv;
use crate::eval::eval_expr::EvalRet;
use crate::eval::r#type::expr::primitive_op::PrimitiveOp;
use crate::eval::r#type::expr::Expr;
use crate::eval::r#type::r#type::Type;
use crate::infra::option::OptionAnyExt;
use crate::infra::r#box::BoxAnyExt;
use crate::infra::result::ResultAnyExt;

mod test;

pub fn case_primitive_op(
    expr_env: &ExprEnv,
    type_annot: &Type,
    op: &Box<PrimitiveOp>,
    eval_env: &Option<Box<ExprEnv>>
) -> EvalRet {
    let eval_env = match eval_env {
        Some(env) => env.clone(),
        None => expr_env.clone().boxed()
    };

    Expr::PrimitiveOp(type_annot.clone(), op.clone(), eval_env.some())
        .ok()
}
