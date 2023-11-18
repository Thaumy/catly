use std::rc::Rc;

use crate::eval::env::expr_env::ExprEnv;
use crate::eval::eval_expr::EvalRet;
use crate::eval::r#type::expr::Expr;
use crate::eval::r#type::r#type::Type;
use crate::infra::option::WrapOption;
use crate::infra::result::ResultAnyExt;

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
    .ok()
}
