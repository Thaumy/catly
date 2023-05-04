mod case_ri;
mod case_t_rc;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::case::cond::infer_branch_type::case_ri::case_ri;
use crate::infer::infer_type::case::cond::infer_branch_type::case_t_rc::case_t_rc;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infra::option::OptionAnyExt;
use crate::infra::r#box::BoxAnyExt;
use crate::infra::triple::Triple;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::OptType;

pub fn infer_branch_type(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &OptType,
    typed_bool_expr: Expr,
    then_expr: &Expr,
    else_expr: &Expr
) -> InferTypeRet {
    match then_expr
        .with_opt_fallback_type(expect_type)
        .infer_type(type_env, expr_env)?
    {
        then_expr_type @ (Triple::L(_) | Triple::M(_)) => {
            let (then_expr_type, constraint_acc, typed_then_expr) =
                then_expr_type.unwrap_type_constraint_expr();

            let new_expr_env = &expr_env
                .extend_constraint_new(constraint_acc.clone());

            case_t_rc(
                type_env,
                new_expr_env,
                then_expr_type,
                expect_type,
                else_expr,
                |type_annot, typed_else_expr| {
                    Expr::Cond(
                        type_annot.some(),
                        typed_bool_expr
                            .clone()
                            .boxed(),
                        typed_then_expr
                            .clone()
                            .boxed(),
                        typed_else_expr.boxed()
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
