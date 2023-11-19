mod on_has_expect_type;
mod on_no_expect_type;

use std::rc::Rc;

use super::case_t_rc::on_has_expect_type::on_has_expect_type;
use super::case_t_rc::on_no_expect_type::on_no_expect_type;
use super::r#fn::{
    destruct_match_const_to_expr_env_inject,
    is_case_expr_valid
};
use crate::infer::env::ExprEnv;
use crate::infer::env::TypeEnv;
use crate::infer::infer_type::InferTypeRet;
use crate::infer::infer_type::TypeMissMatch;
use crate::infra::VecExt;
use crate::infra::WrapQuad;
use crate::infra::WrapResult;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::OptType;

pub fn case_t_rc(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    typed_target_expr: Expr,
    expect_type: &OptType,
    case_vec: &[(Expr, Expr)]
) -> InferTypeRet {
    let target_expr_type = typed_target_expr.unwrap_type_annot();

    // 统一 hint, 并求出 case_expr 解构出的常量环境
    let hinted_cases = {
        let vec = case_vec
            .iter()
            .map(|(case_expr, then_expr)| {
                // Hint every case_expr with target_expr_type
                let case_expr =
                    case_expr.with_fallback_type(target_expr_type);
                // Hint every then_expr with expect_type
                let then_expr =
                    then_expr.with_opt_fallback_type(expect_type);

                // 将 case_expr 解构到常量环境, 该环境将在 then_expr 中被使用
                match destruct_match_const_to_expr_env_inject(
                    type_env, &case_expr
                ) {
                    Ok(env_inject) =>
                        (case_expr, env_inject, then_expr).wrap_ok(),
                    Err((new, old)) =>
                        TypeMissMatch::of_dup_capture(old, new)
                            .wrap_quad_r()
                            .wrap_err(),
                }
            })
            .try_fold(vec![], |acc, x| acc.chain_push(x?).wrap_ok());

        match vec {
            Ok(vec) => vec,
            Err(e) => return e
        }
    };

    let typed_case_expr = match is_case_expr_valid(
        type_env,
        target_expr_type,
        hinted_cases
            .iter()
            .map(|(x, y, _)| (x, y))
    ) {
        Ok(typed_case_expr) => typed_case_expr,
        Err(e) => return e.into()
    };

    let case_env_inject_and_then_expr = hinted_cases
        .into_iter()
        .map(|(_, y, z)| (y, z));

    if let Some(expect_type) = expect_type {
        on_has_expect_type(
            type_env,
            expr_env,
            case_env_inject_and_then_expr,
            expect_type.clone(),
            &typed_case_expr,
            &typed_target_expr
        )
    } else {
        on_no_expect_type(
            type_env,
            expr_env,
            case_env_inject_and_then_expr,
            case_vec,
            &typed_target_expr
        )
    }
}
