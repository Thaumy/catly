mod on_has_expect_type;
mod on_no_expect_type;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::case::r#match::case_t_rc::on_has_expect_type::on_has_expect_type;
use crate::infer::infer_type::case::r#match::case_t_rc::on_no_expect_type::on_no_expect_type;
use crate::infer::infer_type::case::r#match::r#fn::{destruct_match_const_to_expr_env_inject, is_case_expr_valid};
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::quad::QuadAnyExt;
use crate::infra::result::ResultAnyExt;
use crate::infra::vec::Ext;
use crate::parser::r#type::r#type::OptType;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::Type;

pub fn case_t_rc(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    target_expr: &Expr,
    target_expr_type: Type,
    constraint_acc: EnvRefConstraint,
    expect_type: &OptType,
    case_vec: &Vec<(Expr, Expr)>
) -> InferTypeRet {
    //Map<Iter<(Expr,Expr)>,fn(&(Expr,Expr)) -> (Expr,Vec<EnvEntry>,Expr)>
    // 统一 hint, 并求出 case_expr 解构出的常量环境
    let hinted_cases = {
        let vec = case_vec
            .iter()
            .map(|(case_expr, then_expr)| {
                // Hint every case_expr with target_expr_type
                let case_expr =
                    case_expr.with_fallback_type(&target_expr_type);
                // Hint every then_expr with expect_type
                let then_expr =
                    then_expr.with_opt_fallback_type(expect_type);

                // 将 case_expr 解构到常量环境, 该环境将在 then_expr 中被使用
                match destruct_match_const_to_expr_env_inject(
                    type_env, &case_expr
                ) {
                    Ok(env_inject) =>
                        (case_expr, env_inject, then_expr).ok(),
                    Err((new, old)) =>
                        return TypeMissMatch::of_dup_capture(
                            old, new
                        )
                        .quad_r()
                        .err(),
                }
            })
            .try_fold(vec![], |acc, x| match x {
                Ok(it) => acc.chain_push(it).ok(),
                Err(e) => e.err()
            });

        match vec {
            Ok(vec) => vec,
            Err(e) => return e
        }
    };

    match is_case_expr_valid(
        type_env,
        &target_expr_type,
        hinted_cases
            .iter()
            .map(|(x, y, _)| (x, y))
    ) {
        Err(e) => return e,
        _ => {}
    }

    if let Some(expect_type) = expect_type {
        on_has_expect_type(
            type_env,
            expr_env,
            constraint_acc,
            hinted_cases.iter(),
            expect_type.clone()
        )
    } else {
        on_no_expect_type(
            type_env,
            expr_env,
            constraint_acc,
            hinted_cases.iter(),
            target_expr,
            case_vec
        )
    }
}
