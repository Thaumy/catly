use std::ops::Not;

use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt as OptAnyExt;
use crate::infra::quad::Quad;
use crate::infra::r#fn::id;
use crate::infra::result::AnyExt as ResAnyExt;
use crate::parser::expr::Expr;
use crate::type_checker::env::expr_env::ExprEnv;
use crate::type_checker::env::type_env::TypeEnv;
use crate::type_checker::get_type::case::r#match::r#fn::destruct_const_to_expr_env_inject;
use crate::type_checker::get_type::get_type;
use crate::type_checker::get_type::r#type::GetTypeReturn;
use crate::unifier::{can_lift, unify};
use crate::{has_type, require_constraint, type_miss_match};

pub fn case_t_rc(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    target_expr_type: GetTypeReturn,
    expect_type: &MaybeType,
    vec: &Vec<(Expr, Expr)>
) -> GetTypeReturn {
    let (target_expr_type, constraint_of_target_expr_type) =
        match target_expr_type {
            Quad::L(t) => (t, vec![]),
            Quad::ML(rc) => (rc.r#type, rc.constraint),
            x => panic!("Impossible target_expr_type: {:?}", x)
        };

    // 统一进行提示, 并求出 case_expr 解构出的常量环境
    let hinted_cases = vec
        .iter()
        .map(|(case_expr, then_expr)| {
            // Hint every case_expr with target_expr_type
            let case_expr = case_expr
                .clone()
                .with_fallback_type(&target_expr_type);
            // Hint every then_expr with expect_type
            let then_expr = then_expr
                .clone()
                .try_with_fallback_type(expect_type);

            // 将 case_expr 解构到常量环境, 该环境将在 then_expr 中被使用
            let case_expr_env_inject =
                destruct_const_to_expr_env_inject(
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
                &ExprEnv::new(vec![]),
                &case_expr
            ) {
                Quad::L(case_expr_type) => can_lift(
                    type_env,
                    &case_expr_type,
                    &target_expr_type
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
                                .any(|(n, _)| n == capture_name)
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
                            &target_expr_type
                        ),
                // 因为 case_expr 已被 target_expr_type hint
                // 所以 case_expr_type 一定有足够的信息求得类型(即便求出的类型不相容)
                // 不可能出现缺乏类型信息的情况
                // 由此也可推断, case_expr_env 中不存在自由类型
                // 所以在下一步取得 then_expr_type 时, 其产生的约束一定作用于外层
                Quad::MR(x) =>
                    panic!("Impossible case_expr_type: {:?}", x),
                // 类型不相容
                _ => false
            }
        })
        .all(id)
        .not()
    {
        return type_miss_match!();
    }

    // 如果 expect_type 存在
    if let Some(expect_type) = expect_type {
        // 在以 expect_type 为 hint 的基础上获取 then_expr_type 并判断其与 expect_type 的相容性
        // 同时收集在获取 then_expr_type 的过程中产生的约束
        let constraint = hinted_cases
            .map(|(_, case_expr_env, then_expr)| {
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
                            Err(type_miss_match!())
                        },
                    // 获取 then_expr_type 时产生了约束, 这些约束一定作用于外层环境
                    // 因为 case_expr 的每一部分都具备完整的类型信息, 参见上面的推导过程
                    Quad::ML(rc) =>
                        if can_lift(type_env, &rc.r#type, expect_type)
                        {
                            rc.constraint.some().ok()
                        } else {
                            Err(type_miss_match!())
                        },
                    // 获取 then_expr_type 时信息不足或类型不匹配, 这些问题无法被解决
                    mr_r => Err(mr_r)
                }
            })
            .fold(vec![].ok(), |acc, constraint| {
                match (acc, constraint) {
                    // 聚合约束
                    (Ok(mut acc), Ok(constraint)) => {
                        constraint
                            .map(|mut vec| acc.append(&mut vec));
                        acc.ok()
                    }
                    (Ok(_), Err(e)) => Err(e),
                    (Err(e), _) => Err(e)
                }
            });

        match constraint {
            Ok(constraint) =>
                if constraint.is_empty() {
                    has_type!(expect_type.clone())
                } else {
                    require_constraint!(
                        expect_type.clone(),
                        vec![
                            constraint_of_target_expr_type,
                            constraint
                        ]
                        .concat()
                    )
                },
            Err(e) => e
        }
    }
    // 如果 expect_type 不存在
    else {
        // 逐一获取 then_expr_type, 并将它们逐个合一, 合一的结果便是 match 表达式的最终类型
        // 同时收集在获取 then_expr_type 的过程中产生的约束
        let final_type_and_constraint = hinted_cases
            .map(|(_, case_expr_env, then_expr)| {
                // 此部分与上方原理相同
                let then_expr_type = get_type(
                    type_env,
                    &expr_env.extend_vec_new(case_expr_env),
                    &then_expr
                );

                match then_expr_type {
                    Quad::L(then_expr_type) =>
                        (then_expr_type, None).ok(),
                    Quad::ML(rc) =>
                        (rc.r#type, Some(rc.constraint)).ok(),
                    mr_r => Err(mr_r)
                }
            })
            .fold((None, vec![]).ok(), |acc, type_and_constraint| {
                match (acc, type_and_constraint) {
                    // 聚合约束
                    (
                        Ok((acc_t, mut acc_vec)),
                        Ok((t, constraint))
                    ) => {
                        match acc_t {
                            // 对于头一个类型, 只需让它成为初始 acc 类型, 并按需复制约束列表
                            None => match constraint {
                                Some(constraint) =>
                                    (t.clone().some(), constraint),
                                None => (t.clone().some(), acc_vec)
                            }
                            .ok(),
                            // 对于之后的每一个类型, 让它和之前 acc 类型合一, 并按需累积约束列表
                            Some(acc_t) =>
                                match unify(type_env, &acc_t, &t) {
                                    Some(new_acc_t) => {
                                        constraint.map(|mut vec| {
                                            acc_vec.append(&mut vec)
                                        });
                                        (new_acc_t.some(), acc_vec)
                                            .ok()
                                    }
                                    None => Err(type_miss_match!())
                                },
                        }
                    }
                    (Ok(_), Err(e)) => Err(e),
                    (Err(e), _) => Err(e)
                }
            });

        match final_type_and_constraint {
            Ok((Some(final_type), constraint)) =>
                if constraint.is_empty() {
                    has_type!(final_type)
                } else {
                    require_constraint!(
                        final_type,
                        vec![
                            constraint_of_target_expr_type,
                            constraint
                        ]
                        .concat()
                    )
                },
            // match 表达式必须具备至少一个 case
            // 此部分在 AST 构造期间就被保证, 所以此情况不可能发生
            Ok((None, _)) => panic!("Match expr no cases"),
            Err(e) => e
        }
    }
}
