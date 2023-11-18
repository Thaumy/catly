use std::rc::Rc;

use crate::eval::env::ExprEnv;
use crate::eval::eval_expr::EvalRet;
use crate::eval::Expr;
use crate::eval::Type;
use crate::infra::WrapOption;
use crate::infra::WrapResult;

#[cfg(test)]
mod test;

pub fn case_closure(
    type_annot: &Type,
    input_name: &Option<String>,
    input_type: &Type,
    output_expr: &Rc<Expr>,
    eval_env: &Rc<ExprEnv>
) -> EvalRet {
    Expr::Closure(
        type_annot.clone(),
        input_name.clone(),
        input_type.clone(),
        output_expr.clone(),
        eval_env.clone().wrap_some()
    )
    .wrap_ok()
}
