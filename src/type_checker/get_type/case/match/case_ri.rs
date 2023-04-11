use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt as OptAnyExt;
use crate::infra::quad::Quad;
use crate::infra::r#box::Ext;
use crate::infra::r#fn::id;
use crate::infra::result::AnyExt as ResAnyExt;
use crate::parser::expr::Expr;
use crate::parser::r#type::Type;
use crate::type_checker::env::expr_env::ExprEnv;
use crate::type_checker::env::type_env::TypeEnv;
use crate::type_checker::get_type::case::r#match::r#fn::destruct_const_to_expr_env_inject;
use crate::type_checker::get_type::r#type::{
    GetTypeReturn,
    RequireInfo
};
use crate::type_checker::get_type::{get_type, get_type_with_hint};
use crate::type_miss_match;
use crate::unifier::unify;

pub fn case_ri(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    require_info: RequireInfo,
    expect_type: &MaybeType,
    target_expr: &Expr,
    vec: &Vec<(Expr, Expr)>
) -> GetTypeReturn {
    // 由于以下推导可能产生错误, 而这些错误没有很好的语义对应已有的错误类型, 所以需要返回原错误
    let original_err = Quad::MR(require_info);

    // 当 case_expr_type 能够合一为某个类型时, 这个类型与 target_expr 将直接相关
    // 此时以该类型为 hint 求 match 表达式类型

    // 首先取得 case_expr 产生的常量环境, 用作后续检查 case_expr 是否是模式匹配意义上的常量
    let iter = vec
        .iter()
        .map(|(case_expr, then_expr)| {
            let case_expr_env_inject =
                destruct_const_to_expr_env_inject(
                    type_env, &case_expr
                );
            (case_expr, case_expr_env_inject, then_expr)
        });

    // 因为 case 总是倾向于在最后出现通配, 所以倒序合一更有效
    // 也可以改进对多种类型的合一方法, 使得有最大的机会合一成功
    let final_case_expr_type_and_constraint = iter
        .clone()
        .rev()
        .map(|(case_expr, case_expr_env_inject, _)| {
            // 不用 hint case_expr, 因为对 target_expr 的类型获取缺乏信息
            match get_type(type_env, expr_env, case_expr) {
                Quad::L(case_expr_type) => case_expr_type.ok(),
                Quad::ML(rc) =>
                // 确保 case_expr 是模式匹配意义上的常量, 原理与 case_t_rc 相同
                    if rc
                        .constraint
                        .iter()
                        .map(|(capture_name, _)| {
                            case_expr_env_inject
                                .iter()
                                .any(|(n, _)| n == capture_name)
                        })
                        .all(id)
                    {
                        // 以合一的结果 hint target_expr 可能会产生不同的 case_expr 约束
                        // 这些不同的约束将全部作用于用于捕获匹配值的 EnvRef
                        // 这些 EnvRef 会首先尝试提升到 hint...
                        // 总之, 所有的努力都是对某种可能的推导结果的合法尝试, 因此无需收集约束
                        // 相反, 收集约束并判断这些约束是否与 hint 后产生的约束等同, 可能会限制某些推导可能
                        rc.r#type.ok()
                    } else {
                        // 虽然本质上是 case_expr 非模式匹配常量
                        // 但是实际上还是 target_expr 信息不足所致, 原错误返回之
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
                target_expr.clone().boxed(),
                vec.clone(),
            );
            get_type(type_env, expr_env, &expr)
        }
        _ if let Expr::EnvRef(_, ref_name) = target_expr => {
            // 当 case_expr_type 不能合一时, 如果 target_expr 是 EnvRef
            // 那么在求 then_expr 时可能对产生针对 target_expr 的类型约束
            // 以合一后的约束目标为 hint 求 match 表达式类型

            let hint = iter
                .filter(|(_, case_expr_env_inject, _)|
                    // 过滤出所有不受到 case_expr 解构常量环境同名 EnvRef 影响的 then_expr
                    // 因为这些同名 EnvRef 会覆盖对 match 表达式匹配对象的环境引用
                    // 如果常量环境中不存在名为 ref_name 的注入, 那么 then_expr 约束的 ref_name 便是匹配目标
                    case_expr_env_inject
                        .iter()
                        .all(|(n, _)| n != ref_name)
                )
                .map(|(_, case_expr_env_inject, then_expr)|
                    match get_type_with_hint(
                        type_env,
                        // 使用 then_expr 的旁路推导需要来自 case_expr 的常量环境注入
                        // 因为 case_expr 可能包含在 then_expr 中会使用的类型信息
                        // 如果不进行注入, 推导可能会因为缺乏类型信息而失败
                        // let case 的旁路推导因为 assign_type 和 assign_expr 均无法提供有效的类型信息, 所以不需要注入
                        &expr_env.extend_vec_new(case_expr_env_inject),
                        then_expr,
                        expect_type,
                    ) {
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
                        target_expr.clone().boxed(),
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
