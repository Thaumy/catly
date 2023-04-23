use std::collections::HashMap;

use crate::env::expr_env::ExprEnv;
use crate::env::r#type::env_ref_src::EnvRefSrc;
use crate::env::r#type::type_constraint::TypeConstraint;
use crate::env::r#type::type_env::TypeEnv;
use crate::infer_type::r#fn::destruct_namely_type;
use crate::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::option::AnyExt as OptAnyExt;
use crate::infra::quad::Quad;
use crate::infra::r#fn::id;
use crate::infra::result::AnyExt as ResAnyExt;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::Type;

type EnvItem = (String, TypeConstraint, EnvRefSrc);

// 将模式匹配意义上的常量表达式解构为表达式环境注入
pub fn destruct_match_const_to_expr_env_inject<'t>(
    type_env: &TypeEnv,
    expr: &Expr
) -> Vec<EnvItem> {
    // 由于后续的 case_expr_type 会和 target_expr_type 进行相容性测试, 所以这里不负责类型检查
    // 另外在此处实施类型检查是极其复杂的, 这意味着要实现 infer_type 的大部分功能
    match expr {
        Expr::EnvRef(mt, n) => {
            let tc = mt
                .as_ref()
                .map(|t| t.clone().into())
                .unwrap_or(TypeConstraint::Free);

            vec![(n.to_string(), tc, EnvRefSrc::NoSrc)]
        }
        Expr::Struct(t, vec) => {
            // 由于这里不负责类型检查, 所以可以转为无序的哈希表以提升检索效率
            let prod_fields = t.as_ref().and_then(|t| {
                match destruct_namely_type(type_env, &t) {
                    Some(Type::ProdType(vec)) =>
                        HashMap::<String, Type>::from_iter(
                            vec.iter()
                                .map(|(k, v)| (k.clone(), v.clone()))
                        )
                        .some(),
                    _ => None
                }
            });

            vec.iter()
                .map(|(n, mt, e)| {
                    // 简单地从 ProdType 中查找类型作为提示, 因为这里不负责类型检查
                    let prod_hint = prod_fields
                        .as_ref()
                        .and_then(|fields| fields.get(n).cloned());

                    let e = e
                        .with_optional_fallback_type(&prod_hint)
                        .with_optional_fallback_type(&mt);

                    destruct_match_const_to_expr_env_inject(
                        type_env, &e
                    )
                })
                .flatten()
                .collect(): Vec<_>
        }
        _ => vec![]
    }
}

pub fn is_case_expr_valid(
    type_env: &TypeEnv,
    target_expr_type: &Type,
    case_expr_and_env_inject: impl Iterator<Item = (Expr, Vec<EnvItem>)>
) -> Result<(), InferTypeRet> {
    // 逐一确认 case_expr_type 与 target_expr_type 的相容性
    // 同时确保 case_expr 是模式匹配意义上的常量
    case_expr_and_env_inject
        .into_iter()
        .map(|(case_expr, env_inject)| {
            // 使用空表达式环境提取 case_expr_type, 这样能让所有对外界的约束得以暴露
            match case_expr.infer_type(
                type_env,
                &ExprEnv::empty(type_env.clone())
            ) {
                Quad::L(case_expr_type) =>
                    InferTypeRet::from_auto_lift(
                        type_env,
                        &case_expr_type,
                        &target_expr_type
                            .clone()
                            .some(),
                        None
                    ),
                // 表达式环境为空却产生了约束
                Quad::ML(rc) => {
                    let is_constraint_valid = rc
                        .constraint
                        .iter()
                        .map(|(capture_name, _)| {
                            // 这些约束应该全部存在于从常量解构出来的环境中
                            // 它们代表了匹配到的值的捕获
                            // 这些捕获将在 then_expr 的环境中被使用
                            env_inject
                                .iter()
                                .any(|(n, ..)| n == capture_name)
                        })
                        // 如果产生了不存在于常量环境中的约束
                        // 则表明这些约束试图作用于真实的外层环境
                        // 此时的 case_expr 不再是模式匹配意义上可以使用的常量
                        // 模式匹配意义上的常量和一般的常量有所不同
                        // 它允许存在某个用于捕获匹配值的 EnvRef
                        .all(id);

                    if is_constraint_valid {
                        InferTypeRet::from_auto_lift(
                            type_env,
                            &rc.r#type,
                            &target_expr_type
                                .clone()
                                .some(),
                            None
                        )
                    } else {
                        TypeMissMatch::of(&format!(
                            "Case expr not const"
                        ))
                        .into()
                    }
                }
                // 因为 case_expr 已被 target_expr_type hint
                // 所以 case_expr_type 一定有足够的信息求得类型(即便求出的类型不相容)
                // 不可能出现缺乏类型信息的情况
                // 由此也可推断, case_expr_env 中不存在自由类型
                // 所以在下一步取得 then_expr_type 时, 其产生的约束一定作用于外层
                Quad::MR(ri) => panic!("Impossible branch: {ri:?}"),

                // 类型不相容
                x => x
            }
        })
        .find(|x| matches!(x, Quad::R(..)))
        .map(|x| x.err())
        .unwrap_or(Ok(()))
}
