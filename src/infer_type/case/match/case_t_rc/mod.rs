mod case_has_expect_type;
mod case_no_expect_type;

use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::infer_type::case::r#match::case_t_rc::case_has_expect_type::case_has_expect_type;
use crate::infer_type::case::r#match::case_t_rc::case_no_expect_type::case_no_expect_type;
use crate::infer_type::case::r#match::r#fn::{
    destruct_match_const_to_expr_env_inject,
    is_case_expr_valid
};
use crate::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infra::alias::MaybeType;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::Type;

pub fn case_t_rc(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    target_expr: &Expr,
    target_expr_type: Type,
    constraint_acc: EnvRefConstraint,
    expect_type: &MaybeType,
    case_vec: &Vec<(Expr, Expr)>
) -> InferTypeRet {
    //Map<Iter<(Expr,Expr)>,fn(&(Expr,Expr)) -> (Expr,Vec<EnvEntry>,Expr)>
    // 统一 hint, 并求出 case_expr 解构出的常量环境
    let hinted_cases =
        case_vec
            .iter()
            .map(|(case_expr, then_expr)| {
                // Hint every case_expr with target_expr_type
                let case_expr =
                    case_expr.with_fallback_type(&target_expr_type);
                // Hint every then_expr with expect_type
                let then_expr =
                    then_expr.with_opt_fallback_type(expect_type);

                // 将 case_expr 解构到常量环境, 该环境将在 then_expr 中被使用
                let env_inject =
                    destruct_match_const_to_expr_env_inject(
                        type_env, &case_expr
                    );

                (case_expr, env_inject, then_expr)
            });

    match is_case_expr_valid(
        type_env,
        &target_expr_type,
        hinted_cases
            .clone()
            .map(|(x, y, _)| (x, y))
    ) {
        Err(e) => return e,
        _ => {}
    }

    if let Some(expect_type) = expect_type {
        case_has_expect_type(
            type_env,
            expr_env,
            constraint_acc,
            hinted_cases,
            expect_type.clone()
        )
    } else {
        case_no_expect_type(
            type_env,
            expr_env,
            constraint_acc,
            hinted_cases,
            target_expr,
            case_vec
        )
    }
}
