use crate::infer::env::expr_env::{ExprEnv, ExprEnvEntry};
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::require_info::ReqInfo;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::option::OptionAnyExt;
use crate::infra::quad::{Quad, QuadAnyExt};
use crate::infra::r#box::BoxAnyExt;
use crate::infra::result::ResultAnyExt;
use crate::parser::expr::r#type::Expr;

pub fn on_no_expect_type<T>(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    case_env_inject_and_then_expr: T,
    case_vec: &Vec<(Expr, Expr)>,
    typed_target_expr: &Expr
) -> InferTypeRet
where
    T: Iterator<Item = (Vec<ExprEnvEntry>, Expr)> + Clone
{
    let case_env_inject_and_then_expr =
        case_env_inject_and_then_expr.into_iter();

    // 逐一获取 then_expr_type, 并将它们逐个合一, 合一的结果便是 match 表达式的最终类型
    // 同时收集在获取 then_expr_type 的过程中产生的约束
    let then_expr_type_and_outer_constraints =
        case_env_inject_and_then_expr.map(
            |(env_inject, then_expr)| {
                // 此部分与 on_has_expect_type 原理相同
                match then_expr.infer_type(
                    type_env,
                    &expr_env.extend_vec_new(env_inject.clone())
                ) {
                    result @ (Quad::L(_) | Quad::ML(_)) => {
                        // 此处不负责对类型完备的 then_expr 进行收集
                        // 因为即便收集, on_has_expect_type 也要进行重复的收集工作
                        // 但更多是出于实现的复杂性考虑
                        let (typed_then_expr, constraint) = result
                            .unwrap_expr_constraint()
                            .clone();
                        let then_expr_type = typed_then_expr
                            .unwrap_type_annot()
                            .clone();

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
                    Quad::MR(ri) => ReqInfo::of(
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
            }
        );

    // 一旦发现类型不匹配(of then_expr), 立即返回
    match then_expr_type_and_outer_constraints
        .clone()
        // 任选一个错误即可(渐进式错误提示)
        .find(|x| matches!(x, Err(Quad::R(_))))
    {
        Some(Err(type_miss_match)) => return type_miss_match,
        _ => {}
    } // 排除了 infer_type 的结果 R

    let outer_constraint = then_expr_type_and_outer_constraints
        .clone()
        .try_fold(EnvRefConstraint::empty(), |acc, x| match x {
            Ok((_, c)) => match acc.extend_new(c.clone()) {
                Some(acc) => acc.ok(),
                None => TypeMissMatch::of_constraint(&acc, &c).err()
            },
            Err(Quad::MR(ri)) => match acc
                .extend_new(ri.constraint.clone())
            {
                Some(acc) => acc.ok(),
                None =>
                    TypeMissMatch::of_constraint(&acc, &ri.constraint)
                        .err(),
            },
            _ => acc.ok()
        });

    // 如果合并约束时发生冲突, 立即返回
    let outer_constraint = match outer_constraint {
        Ok(c) => c,
        Err(type_miss_match) => return type_miss_match.into()
    };

    // 由于缺乏 expect_type, 可能有一部分 then_expr 无法获得类型
    // 需要去除这些无法获得类型的 then_expr, 将剩余类型合一后 hint match match expr
    // 由于 R 情况已被排除, 此处需要排除 MR 情况, 所以仅保留 Ok 即可
    let final_type = then_expr_type_and_outer_constraints
        .filter_map(|x| x.ok())
        .map(|(t, _)| t)
        .try_reduce(|acc, t| match acc.unify(type_env, &t) {
            Some(acc) => acc.ok(),
            None => TypeMissMatch::of_type(&acc, &t)
                .quad_r()
                .err()
        });

    // TODO: what the magic?
    // 为什么不是 Option<Result<Type,InferTypeRet..?
    // 是 ChangeOutputType 的效果嘛?
    let final_type = match final_type {
        Ok(Some(t)) => t,
        // 出现合一错误
        Err(e) => return e,
        // 所有 then_expr 都缺乏信息
        Ok(None) =>
            return ReqInfo::of("(then expr)", outer_constraint)
                .quad_mr(),
    };

    let match_expr = Expr::Match(
        final_type.some(),
        typed_target_expr
            .clone()
            .boxed(),
        case_vec.clone()
    );

    let new_expr_env =
        expr_env.extend_constraint_new(outer_constraint.clone());

    match_expr
        .infer_type(type_env, &new_expr_env)?
        .with_constraint_acc(outer_constraint)
}
