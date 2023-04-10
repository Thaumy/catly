use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt as OptAnyExt;
use crate::infra::quad::Quad;
use crate::infra::r#box::Ext;
use crate::infra::r#fn::id;
use crate::infra::result::AnyExt as ResAnyExt;
use crate::parser::expr::Expr;
use crate::parser::r#type::Type;
use crate::type_checker::get_type::case::r#match::r#fn::destruct_const_to_expr_env;
use crate::type_checker::get_type::r#type::{
    ExprEnv,
    GetTypeReturn,
    RequireInfo,
    TypeEnv
};
use crate::type_checker::get_type::{get_type, get_type_with_hint};
use crate::type_miss_match;
use crate::unifier::unify;

pub fn case_ri(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    require_info: RequireInfo,
    expect_type: &MaybeType,
    match_expr: &Expr,
    vec: &Vec<(Expr, Expr)>
) -> GetTypeReturn {
    let original_err = Quad::MR(require_info);

    // 当 case_expr_type 能够合一为某个类型时, 这个类型与 match_expr 将直接相关
    // 此时以该类型为 hint 求 match 表达式类型

    // 首先取得 case_expr 产生的常量环境, 用作后续检查 case_expr 是否是模式匹配意义上的常量
    let iter = vec
        .iter()
        .map(|(case_expr, then_expr)| {
            let case_expr_env =
                destruct_const_to_expr_env(type_env, &case_expr);
            (case_expr, case_expr_env, then_expr)
        });

    // 因为 case 总是倾向于在最后出现通配, 所以倒序合一更有效
    // 也可以改进对多种类型的合一方法, 使得有最大的机会合一成功
    let final_case_expr_type_and_constraint = iter
        .clone()
        .rev()
        .map(|(case_expr, case_expr_env, _)| {
            // 不用 hint, 因为 match_expr 此时无法获取类型
            match get_type(type_env, expr_env, case_expr) {
                Quad::L(case_expr_type) => case_expr_type.ok(),
                Quad::ML(rc) =>
                // 确保 case_expr 是模式匹配意义上的常量, 原理与 case_t_rc 相同
                    if rc
                        .constraint
                        .iter()
                        .map(|(n, _)| {
                            case_expr_env
                                .iter()
                                .any(|(x, _)| n == x)
                        })
                        .all(id)
                    {
                        // 无需收集约束
                        // 如果所有 case_expr 都能取得类型, 说明它们即使被 hint 也能产生相同的约束
                        // 因而无需担心以合一的结果 hint match_expr 会产生不同的 case_expr 约束的问题
                        rc.r#type.ok()
                    } else {
                        // 虽然本质上是 case_expr 非模式匹配常量
                        // 但是实际上还是 match_expr 信息不足所致, 原错误返回之
                        original_err.clone().err()
                    },
                _ => original_err.clone().err() // 原理同上
            }
        })
        .fold(None.ok(), |acc, type_and_constraint| {
            match (acc, type_and_constraint) {
                (Ok(acc), Ok(t)) => {
                    match acc {
                        // 对于头一个类型, 只需让它成为初始 acc 类型
                        None => t.clone().some().ok(),
                        // 对于之后的每一个类型, 让它和之前 acc 类型合一
                        Some(acc) => match unify(type_env, &acc, &t) {
                            Some(new_acc) => new_acc.some().ok(),
                            None => Err(type_miss_match!())
                        }
                    }
                }
                (Ok(_), Err(_)) => original_err.clone().err(),
                (Err(_), _) => original_err.clone().err()
            }
        });

    match final_case_expr_type_and_constraint {
        Ok(t) => {
            let expr = Expr::Match(
                t,
                match_expr.clone().boxed(),
                vec.clone(),
            );
            get_type(type_env, expr_env, &expr)
        }
        _ if let Expr::EnvRef(.., ref_name) = match_expr => {
            // 当 case_expr_type 不能合一时, 如果 match_expr 是 EnvRef
            // 那么在求 then_expr 时可能对产生针对 match_expr 的类型约束
            // 以合一后的约束目标为 hint 求 match 表达式类型

            // 过滤出所有不受到 case_expr 解构常量环境同名 EnvRef 影响的 then_expr
            // 因为这些同名 EnvRef 会覆盖对 match 表达式匹配对象的环境引用
            let hint = iter
                .filter(|(_, case_expr_env, _)|
                    case_expr_env.iter().all(|(n, _)| n != ref_name)
                )
                .map(|(_, _, then_expr)|
                    match get_type_with_hint(type_env, expr_env, then_expr, expect_type) {
                        Quad::ML(rc) => rc.constraint
                            .iter()
                            .find(|(n, _)| n == ref_name)
                            .map(|(_, t)| t.clone()),
                        _ => None
                    }
                ).fold(None: Option<Type>, |acc, t|
                match (acc, t) {
                    (Some(acc), Some(t)) => unify(type_env, &acc, &t),
                    (Some(acc), None) => Some(acc),
                    _ => None
                },
            );

            match hint {
                Some(t) => {
                    let expr = Expr::Match(
                        t.some(),
                        match_expr.clone().boxed(),
                        vec.clone(),
                    );
                    get_type(type_env, expr_env, &expr)
                }
                None => original_err
            }
        }
        _ => original_err
    }
}
