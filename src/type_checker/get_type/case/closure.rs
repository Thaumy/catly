use std::ops::Deref;

use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt as OptAnyExt;
use crate::infra::quad::Quad;
use crate::infra::r#box::Ext as BoxExt;
use crate::infra::vec::Ext as VecExt;
use crate::parser::expr::Expr;
use crate::parser::r#type::Type;
use crate::type_checker::get_type::get_type_with_hint;
use crate::type_checker::get_type::r#fn::{
    destruct_type_env_ref,
    lift_or_left,
    with_constraint_lift_or_left
};
use crate::type_checker::get_type::r#type::{
    ExprEnv,
    GetTypeReturn,
    TypeConstraint,
    TypeEnv
};
use crate::{discard_type, has_type, type_miss_match};

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    t: &MaybeType,
    input_name: &Option<String>,
    input_type: &MaybeType,
    output_expr: &Expr
) -> GetTypeReturn {
    // Destruct t to ClosureType
    let (t_i_t, t_o_t) = match t {
        Some(t) => match destruct_type_env_ref(type_env, t) {
            Some(Type::ClosureType(t_i_t, t_o_t)) => (
                t_i_t
                    .clone()
                    .deref()
                    .clone()
                    .some(),
                t_o_t
                    .clone()
                    .deref()
                    .clone()
                    .some()
            ),
            _ => return type_miss_match!()
        },
        _ => (None, None)
    };

    // Hint input_type
    let input_type = match input_type {
        None => t_i_t,
        t => t.clone()
    };

    // TODO: fn env_inject
    // Inject parameter to env
    let expr_env = match input_name {
        Some(input_name) => expr_env.push_to_new((
            input_name.to_string(),
            input_type
                .as_ref()
                .map(|t| TypeConstraint::Constraint(t.clone()))
                .unwrap_or_else(|| TypeConstraint::Free)
        )),
        _ => expr_env.clone()
    };

    // Hint and get output_expr_type
    let output_expr_type =
        get_type_with_hint(type_env, &expr_env, output_expr, &t_o_t);

    // 此处并不将 output_expr_type 与 t_o_t(即hint) 进行相容性判断
    // 同时判断只能让一部分情况提前返回
    // 因为对于两个 output_type 相容的 Closure, 其整体可能是不相容的(即使 input_type 也相容)
    // 在此处进行相容性判断可能会让已有的相容规则被破坏
    // 由此可以得出泛化的结论:
    // 对于所有由类型提示得出的类型, 都不应该与类型提示进行相容性判断而提前返回, 因为这种判断并非类型的最终合一规则

    match output_expr_type {
        Quad::L(output_expr_type) => {
            let base = match input_type {
                Some(input_type) => Type::ClosureType(
                    input_type.clone().boxed(),
                    output_expr_type.boxed()
                ),
                // 输入类型自由, 而 output_expr_type 不需要约束, 说明不需要输入类型
                // 因为类型和值绑定, 所以 output_expr 和输入无关
                // 实际上这和弃元输入值等效
                // TODO: Discard -> X 的 lift 特例
                None => Type::ClosureType(
                    discard_type!().boxed(),
                    output_expr_type.boxed()
                )
            };

            // Lift inferred ClosureType to t
            match lift_or_left(type_env, &base, t) {
                Some(t) => has_type!(t),
                None => type_miss_match!()
            }
        }
        Quad::ML(rc) => {
            match input_name {
                Some(input_name) => match input_type {
                    // 因为输入具名有类型, 所以约束不可能包含自输入
                    // 换言之, 输入在推导 output_expr_type 之前就已经被约束了
                    // 此时只需要提升类型, 并将约束传播
                    Some(input_type) => {
                        let base = Type::ClosureType(
                            input_type.clone().boxed(),
                            rc.r#type.boxed()
                        );

                        with_constraint_lift_or_left(
                            rc.constraint,
                            type_env,
                            &base,
                            t
                        )
                    }
                    // 输入具名无类型
                    // 如果约束包含了输入, 需把输入类型限定到约束目标, 并将其从约束列表中移除
                    // 然后传播剩余约束(仍有剩余约束), 或返回确切类型(不存在剩余约束)
                    None => {
                        let input_type_constraint = rc
                            .constraint
                            .iter()
                            .rev()
                            .find(|(n, _)| n == input_name)
                            .map(|(_, t)| t);

                        if let Some(input_type_constraint) =
                            input_type_constraint
                        {
                            // 约束包含输入, 需要限定输入类型到约束目标并将其从约束列表中移除
                            // 将剩余约束过滤出来
                            let constraint = rc
                                .constraint
                                .iter()
                                .filter(|(n, _)| n != input_name)
                                .map(|x| x.clone())
                                .collect():
                                Vec<_>;

                            let base = Type::ClosureType(
                                input_type_constraint
                                    .clone()
                                    .boxed(),
                                rc.r#type.boxed()
                            );

                            with_constraint_lift_or_left(
                                constraint, type_env, &base, t
                            )
                        } else {
                            // 约束不包含输入, 只需将其传播
                            let base = Type::ClosureType(
                                discard_type!().boxed(),
                                rc.r#type.boxed()
                            );

                            with_constraint_lift_or_left(
                                rc.constraint,
                                type_env,
                                &base,
                                t
                            )
                        }
                    }
                },
                None => {
                    // 输入被弃元, 但 output_expr_type 需要约束
                    // 说明约束目标是外层环境
                    let base = Type::ClosureType(
                        discard_type!().boxed(),
                        rc.r#type.boxed()
                    );

                    with_constraint_lift_or_left(
                        rc.constraint,
                        type_env,
                        &base,
                        t
                    )
                }
            }
        }

        // 不能推导出输出类型(即便进行了类型提示), 或推导错误
        // 推导错误是由类型不匹配导致的, 这种错误无法解决
        // 不能推导出输出类型是由缺乏类型信息导致的
        // 因为 Closure 不存在可以推导输出类型的第二个表达式, 所以不适用于旁路类型推导
        mr_r => mr_r.clone()
    }
}
