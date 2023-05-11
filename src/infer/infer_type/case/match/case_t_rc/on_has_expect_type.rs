use std::rc::Rc;

use crate::infer::env::expr_env::{ExprEnv, ExprEnvEntry};
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::require_constraint::require_constraint;
use crate::infer::infer_type::r#type::require_info::ReqInfo;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::option::OptionAnyExt;
use crate::infra::quad::{Quad, QuadAnyExt};
use crate::infra::rc::RcAnyExt;
use crate::infra::result::ResultAnyExt;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::Type;

pub fn on_has_expect_type<T>(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    case_env_inject_and_then_expr: T,
    expect_type: Type,
    typed_case_expr: &[Expr],
    typed_target_expr: &Expr
) -> InferTypeRet
where
    T: Iterator<Item = (Vec<ExprEnvEntry>, Expr)> + Clone
{
    // 在以 expect_type 为 hint 的基础上获取 then_expr_type 并判断其与 expect_type 的相容性
    // 同时收集在获取 then_expr_type 的过程中产生的约束
    let typed_then_expr_and_outer_constraints =
        case_env_inject_and_then_expr.map(
            |(env_inject, then_expr)| {
                // 此处 then_expr 已由上方统一 hint
                let result = then_expr.infer_type(
                    type_env,
                    // then_expr 需要在原环境和常量环境的拼接中求类型
                    &expr_env.extend_vec_new(env_inject.clone())
                );

                // 虽然将每次产生的外界约束注入环境有助于获取下一次的类型
                // 但这样做的效率并不高, 因为 match 可能有很多分支, 且难以预测最佳的注入顺序(这貌似是个NP问题?
                // 交由分支约束共享逻辑可简化实现, 并具备可观的性能

                match result {
                    // 如果获取 then_expr_type 时产生了约束, 这些约束一定作用于外层环境
                    // 因为 case_expr 的每一部分都具备完整的类型信息, 参见上面的推导过程
                    Quad::L(_) | Quad::ML(_) => {
                        let (typed_then_expr, constraint) =
                            result.unwrap_expr_constraint();

                        let then_expr_type =
                            typed_then_expr.unwrap_type_annot();

                        // 将作用于常量环境的约束过滤掉, 收集外部约束用于分支共享
                        let outer_constraint =
                            constraint.filter_new(|(n, _)| {
                                // 如果约束到任何匹配捕获, 则视为内部约束
                                !env_inject.iter().any(
                                    |(capture_name, ..)| {
                                        capture_name == n
                                    }
                                )
                            });

                        if then_expr_type
                            .can_lift_to(type_env, &expect_type)
                        {
                            (typed_then_expr, outer_constraint).ok()
                        } else {
                            TypeMissMatch::of_type(
                                then_expr_type,
                                &expect_type
                            )
                            .quad_r()
                            .err()
                        }
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
    if let Some(Err(type_miss_match)) =
        typed_then_expr_and_outer_constraints
            .clone()
            // 任选一个错误即可(渐进式错误提示)
            .find(|x| matches!(x, Err(Quad::R(_))))
    {
        return type_miss_match;
    } // 排除了 infer_type 的结果 R

    let outer_constraint = typed_then_expr_and_outer_constraints
        .clone()
        // 与累积约束合并
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

    // 如果出现缺乏类型信息(of then_expr), 则将收集到的外部约束传播出去
    if let Some(Err(Quad::MR(ri))) =
        typed_then_expr_and_outer_constraints
            .clone()
            .find(|x| matches!(x, Err(Quad::MR(_))))
    {
        return ReqInfo::of(ri.ref_name, outer_constraint).into();
    } // 排除了 infer_type 的结果 MR

    let typed_then_expr = typed_then_expr_and_outer_constraints
        .filter_map(|x| x.ok())
        .map(|(e, _)| e);

    let typed_cases = typed_case_expr
        .iter()
        // 在此处类型检查已经完成, 不会出现无法配对的情况
        .zip(typed_then_expr)
        .map(|(x, y)| (x.clone(), y))
        .collect(): Vec<_>;

    require_constraint(
        Expr::Match(
            expect_type.some(),
            typed_target_expr.clone().rc(),
            typed_cases
        ),
        outer_constraint
    )
}
