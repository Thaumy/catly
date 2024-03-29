mod case_ri;
mod case_t_rc;

use std::rc::Rc;

use case_ri::case_ri;
use case_t_rc::case_t_rc;

use crate::infer::env::ExprEnv;
use crate::infer::env::TypeEnv;
use crate::infer::infer_type::InferTypeRet;
use crate::infra::Triple;
use crate::infra::WrapOption;
use crate::infra::WrapRc;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::OptType;

pub fn infer_branch_type(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    expect_type: &OptType,
    typed_bool_expr: Expr,
    then_expr: &Expr,
    else_expr: &Expr
) -> InferTypeRet {
    match then_expr
        .with_opt_fallback_type(expect_type)
        .infer_type(type_env, expr_env)?
    {
        result @ (Triple::L(_) | Triple::M(_)) => {
            let (typed_then_expr, constraint_acc) =
                result.unwrap_expr_constraint();

            let new_expr_env = &expr_env
                .extend_constraint_new(constraint_acc.clone());

            case_t_rc(
                type_env,
                new_expr_env,
                typed_then_expr
                    .unwrap_type_annot()
                    .clone(),
                expect_type,
                else_expr,
                |type_annot, typed_else_expr| {
                    Expr::Cond(
                        type_annot.wrap_some(),
                        typed_bool_expr
                            .clone()
                            .wrap_rc(),
                        typed_then_expr
                            .clone()
                            .wrap_rc(),
                        typed_else_expr.wrap_rc()
                    )
                }
            )?
            .with_constraint_acc(constraint_acc)
        }

        Triple::R(ri)
            if then_expr.is_no_type_annot() &&
                expect_type.is_none() =>
        {
            let constraint_acc = ri.constraint;

            let new_expr_env = &expr_env
                .extend_constraint_new(constraint_acc.clone());

            case_ri(
                type_env,
                new_expr_env,
                &typed_bool_expr,
                else_expr,
                then_expr
            )?
            .with_constraint_acc(constraint_acc)
        }

        Triple::R(ri) => ri.into()
    }
}
