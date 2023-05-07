use std::rc::Rc;

use crate::eval::env::expr_env::ExprEnv;
use crate::eval::eval_expr::EvalRet;
use crate::eval::r#type::expr::Expr;
use crate::eval::r#type::r#type::Type;
use crate::infra::option::OptionAnyExt;
use crate::infra::result::ResultAnyExt;

#[cfg(test)]
mod test;

pub fn case_closure(
    expr_env: Rc<ExprEnv>,
    type_annot: &Type,
    input_name: &Option<String>,
    input_type: &Type,
    output_expr: &Box<Expr>,
    eval_env: &Option<Rc<ExprEnv>>
) -> EvalRet {
    let eval_env = match eval_env {
        Some(env) => env.clone(),
        None => expr_env
    };

    Expr::Closure(
        type_annot.clone(),
        input_name.clone(),
        input_type.clone(),
        output_expr.clone(),
        eval_env.some()
    )
    .ok()
}
