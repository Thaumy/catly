use std::ops::Not;

use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::get_type::case::r#match::r#fn::destruct_match_const_to_expr_env_inject;
use crate::get_type::get_type;
use crate::get_type::r#fn::{has_type, require_constraint_or_type};
use crate::get_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::get_type::r#type::require_constraint::require_extended_constraint;
use crate::get_type::r#type::type_miss_match::TypeMissMatch;
use crate::get_type::r#type::GetTypeReturn;
use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt as OptAnyExt;
use crate::infra::quad::Quad;
use crate::infra::r#fn::id;
use crate::infra::result::AnyExt as ResAnyExt;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::Type;
use crate::unify::{can_lift, unify};

pub fn case_t_rc(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    target_expr_type: Type,
    constraint_acc: EnvRefConstraint,
    expect_type: &MaybeType,
    vec: &Vec<(Expr, Expr)>
) -> GetTypeReturn {
    // 统一 hint, 并求出 case_expr 解构出的常量环境
    let hinted_cases = vec
        .iter()
        .map(|(case_expr, then_expr)| {
            // Hint every case_expr with target_expr_type
            let case_expr =
                case_expr.with_fallback_type(&target_expr_type);
            // Hint every then_expr with expect_type
            let then_expr =
                then_expr.try_with_fallback_type(expect_type);

            // 将 case_expr 解构到常量环境, 该环境将在 then_expr 中被使用
            let case_expr_env_inject =
                destruct_match_const_to_expr_env_inject(
                    type_env, &case_expr
                );

            (case_expr, case_expr_env_inject, then_expr)
        });

    // 逐一确认 case_expr_type 与 target_expr_type 的相容性
    // 同时确保 case_expr 是模式匹配意义上的常量
    if hinted_cases
        .clone()
        .map(|(case_expr, case_expr_env_inject, _)| {
            // 使用空表达式环境提取 case_expr_type, 这样能让所有对外界的约束得以暴露
            match get_type(
                type_env,
                &ExprEnv::new(type_env.clone(), vec![]),
                &case_expr,
            ) {
                Quad::L(case_expr_type) => can_lift(
                    type_env,
                    &case_expr_type,
                    &target_expr_type,
                ),
                // 表达式环境为空却产生了约束
                Quad::ML(rc) =>
                    rc.constraint
                        .iter()
                        .map(|(capture_name, _)| {
                            // 这些约束应该全部存在于从常量解构出来的环境中
                            // 它们代表了匹配到的值的捕获
                            // 这些捕获将在 then_expr 的环境中被使用
                            case_expr_env_inject
                                .iter()
                                .any(|(n, ..)| n == capture_name)
                        })
                        // 如果产生了不存在于常量环境中的约束
                        // 则表明这些约束试图作用于真实的外层环境
                        // 此时的 case_expr 不再是模式匹配意义上可以使用的常量
                        // 模式匹配意义上的常量和一般的常量有所不同
                        // 它允许存在某个用于捕获匹配值的 EnvRef
                        .all(id) &&
                        can_lift(
                            type_env,
                            &rc.r#type,
                            &target_expr_type,
                        ),
                // 因为 case_expr 已被 target_expr_type hint
                // 所以 case_expr_type 一定有足够的信息求得类型(即便求出的类型不相容)
                // 不可能出现缺乏类型信息的情况
                // 由此也可推断, case_expr_env 中不存在自由类型
                // 所以在下一步取得 then_expr_type 时, 其产生的约束一定作用于外层
                ri if let Quad::MR(_) = ri =>
                    panic!("Impossible branch: {ri:?}"),

                // 类型不相容
                _ => false
            }
        })
        .all(id)
        .not()
    {
        return TypeMissMatch::of(&format!(
            "Case types <> {target_expr_type:?}"
        ))
        .into();
    }

    // 如果 expect_type 存在
    if let Some(expect_type) = expect_type {
        // 在以 expect_type 为 hint 的基础上获取 then_expr_type 并判断其与 expect_type 的相容性
        // 同时收集在获取 then_expr_type 的过程中产生的约束
        let constraint = hinted_cases
            .map(|(_, case_expr_env, then_expr)| {
                // 此处 then_expr 已由上方统一 hint
                let then_expr_type = get_type(
                    type_env,
                    // then_expr 需要在原环境和常量环境的拼接中求类型
                    &expr_env.extend_vec_new(case_expr_env),
                    &then_expr
                );

                match then_expr_type {
                    Quad::L(then_expr_type) =>
                        if can_lift(
                            type_env,
                            &then_expr_type,
                            expect_type
                        ) {
                            None.ok()
                        } else {
                            Quad::R(TypeMissMatch::of_type(
                                &then_expr_type,
                                &expect_type
                            ))
                            .err()
                        },
                    // 获取 then_expr_type 时产生了约束, 这些约束一定作用于外层环境
                    // 因为 case_expr 的每一部分都具备完整的类型信息, 参见上面的推导过程
                    Quad::ML(rc) =>
                        if can_lift(type_env, &rc.r#type, expect_type)
                        {
                            rc.constraint.some().ok()
                        } else {
                            Quad::R(TypeMissMatch::of_type(
                                &rc.r#type,
                                &expect_type
                            ))
                            .err()
                        },
                    // 获取 then_expr_type 时信息不足或类型不匹配, 这些问题无法被解决
                    mr_r => Err(mr_r)
                }
            })
            .fold(
                EnvRefConstraint::empty().ok(),
                |acc, constraint| {
                    match (acc, constraint) {
                        // 无约束
                        (Ok(acc), Ok(None)) => acc.ok(),
                        // 聚合约束
                        (Ok(acc), Ok(Some(constraint))) => match acc
                            .extend_new(constraint.clone())
                        {
                            Some(acc) => acc.ok(),
                            None =>
                                Quad::R(TypeMissMatch::of_constraint(
                                    &acc,
                                    &constraint
                                ))
                                .err(),
                        },
                        (Ok(_), Err(e)) => Err(e),
                        (Err(e), _) => Err(e)
                    }
                }
            );

        match constraint {
            Ok(constraint) =>
                if constraint_acc.is_empty() && constraint.is_empty()
                {
                    has_type(expect_type.clone())
                } else {
                    require_extended_constraint(
                        expect_type.clone(),
                        constraint_acc,
                        constraint.clone()
                    )
                },
            Err(e) => e
        }
    }
    // 如果 expect_type 不存在
    else {
        let mut constraint_acc = constraint_acc;

        // 逐一获取 then_expr_type, 并将它们逐个合一, 合一的结果便是 match 表达式的最终类型
        // 同时收集在获取 then_expr_type 的过程中产生的约束
        let final_type = hinted_cases
            .map(|(_, case_expr_env, then_expr)| {
                // 此部分与上方原理相同
                let then_expr_type = get_type(
                    type_env,
                    &expr_env.extend_vec_new(case_expr_env),
                    &then_expr
                );

                match then_expr_type {
                    Quad::L(then_expr_type) => then_expr_type.ok(),
                    Quad::ML(rc) => match constraint_acc
                        .extend_new(rc.constraint.clone())
                    {
                        Some(constraint) => {
                            constraint_acc = constraint;
                            rc.r#type.ok()
                        }
                        None => Quad::R(TypeMissMatch::of_constraint(
                            &constraint_acc.clone(),
                            &rc.constraint
                        ))
                        .err()
                    },
                    mr_r => Err(mr_r)
                }
            })
            .reduce(|acc, t| match (acc, t) {
                (Ok(acc), Ok(t)) => match unify(type_env, &acc, &t) {
                    Some(acc) => acc.ok(),
                    None => Quad::R(TypeMissMatch::of_type(&acc, &t))
                        .err()
                },
                (Ok(_), Err(e)) => Err(e),
                (Err(e), _) => Err(e)
            })
            // match 表达式至少具备一个 case, 这在 AST 构造期间就被保证
            .unwrap_or_else(|| panic!("Match expr no cases")); // 所以一定能够成功

        match final_type {
            Ok(final_type) =>
                require_constraint_or_type(constraint_acc, final_type),
            Err(e) => e
        }
    }
}
