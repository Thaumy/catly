use crate::env::expr_env::{EnvEntry, ExprEnv};
use crate::env::r#type::type_env::TypeEnv;
use crate::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer_type::r#type::require_constraint::require_constraint;
use crate::infer_type::r#type::require_info::RequireInfo;
use crate::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::option::AnyExt;
use crate::infra::quad::{AnyExt as QuadAnyExt, Quad};
use crate::infra::r#box::Ext;
use crate::infra::result::AnyExt as ResAnyExt;
use crate::parser::expr::r#type::Expr;

pub fn case_no_expect_type<T>(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    constraint_acc: EnvRefConstraint,
    hinted_cases: T,
    target_expr: &Expr,
    case_vec: &Vec<(Expr, Expr)>
) -> InferTypeRet
where
    T: Iterator<Item = (Expr, Vec<EnvEntry>, Expr)> + Clone
{
    let hinted_cases = hinted_cases.into_iter();

    // 逐一获取 then_expr_type, 并将它们逐个合一, 合一的结果便是 match 表达式的最终类型
    // 同时收集在获取 then_expr_type 的过程中产生的约束
    let type_and_outer_constraints =
        hinted_cases.map(|(_, env_inject, then_expr)| {
            // 此部分与上方原理相同
            let then_expr_type = then_expr.infer_type(
                type_env,
                &expr_env.extend_vec_new(env_inject.clone())
            );

            match then_expr_type {
                Quad::L(_) | Quad::ML(_) => {
                    let (then_expr_type, constraint) =
                        then_expr_type.unwrap_type_and_constraint();

                    // 将作用于常量环境的约束过滤掉, 收集外部约束用于分支共享
                    let outer_constraint =
                        constraint.filter_new(|(n, _)| {
                            !env_inject.iter().any(
                                |(capture_name, ..)| {
                                    capture_name == n
                                }
                            )
                        });

                    (then_expr_type, outer_constraint).ok()
                }
                // 同样需要去除对常量环境的约束
                Quad::MR(ri) => RequireInfo::of(
                    ri.ref_name,
                    ri.constraint
                        .filter_new(|(n, _)| {
                            !env_inject.iter().any(
                                |(capture_name, ..)| {
                                    capture_name == n
                                }
                            )
                        })
                )
                .quad_mr()
                .err(),

                // 获取 then_expr_type 时类型不匹配
                r => r.err()
            }
        });

    // 一旦发现类型不匹配(of then_expr), 立即返回
    match type_and_outer_constraints
        .clone()
        // 任选一个错误即可(渐进式错误提示)
        .find(|x| matches!(x, Err(Quad::R(_))))
    {
        Some(Err(type_miss_match)) => return type_miss_match,
        _ => {}
    } // 排除了 infer_type 的结果 R

    let outer_constraint = type_and_outer_constraints
        .clone()
        .fold(constraint_acc.ok(), |acc, x| match (acc, x) {
            (Ok(acc), Ok((_, c))) =>
                match acc.extend_new(c.clone()) {
                    Some(acc) => acc.ok(),
                    None =>
                        TypeMissMatch::of_constraint(&acc, &c).err(),
                },
            (Ok(acc), Err(Quad::MR(ri))) => match acc
                .extend_new(ri.constraint.clone())
            {
                Some(acc) => acc.ok(),
                None =>
                    TypeMissMatch::of_constraint(&acc, &ri.constraint)
                        .err(),
            },
            (Ok(acc), _) => acc.ok(),
            (Err(e), _) => e.err()
        });

    // 如果合并约束时发生冲突, 立即返回
    let outer_constraint = match outer_constraint {
        Ok(c) => c,
        Err(type_miss_match) => return type_miss_match.into()
    };

    // 由于缺乏 expect_type, 可能有一部分 then_expr 无法获得类型
    // 需要去除这些无法获得类型的 then_expr, 将剩余类型合一后 hint match match expr
    // 由于 R 情况已被排除, 此处需要排除 MR 情况, 所以仅保留 Ok 即可
    let final_type = type_and_outer_constraints
        .filter(|x| matches!(x, Ok(_)))
        .map(|x| match x {
            Ok((t, _)) => Ok(t),
            _ => panic!("Impossible value: {x:?}")
        })
        .reduce(|acc, t| match (acc, t) {
            (Ok(acc), Ok(t)) => match acc.unify(type_env, &t) {
                Some(acc) => acc.ok(),
                None => TypeMissMatch::of_type(&acc, &t)
                    .quad_r()
                    .err()
            },
            (Ok(_), Err(e)) => e.err(),
            (Err(e), _) => e.err()
        });

    let final_type = match final_type {
        Some(Ok(t)) => t,
        // 出现合一错误
        Some(Err(e)) => return e,
        // 所有 then_expr 都缺乏信息
        None =>
            return RequireInfo::of("(then expr)", outer_constraint)
                .into(),
    };

    let match_expr = Expr::Match(
        final_type.some(),
        target_expr.clone().boxed(),
        case_vec.clone()
    );

    let new_expr_env =
        expr_env.extend_constraint_new(outer_constraint.clone());

    match match_expr.infer_type(type_env, &new_expr_env) {
        Quad::L(t) => require_constraint(t, outer_constraint),
        Quad::ML(rc) => rc.with_constraint_acc(outer_constraint),
        Quad::MR(ri) => ri.with_constraint_acc(outer_constraint),
        r => r
    }
}
