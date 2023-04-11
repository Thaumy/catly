use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::parser::expr::Expr;
use crate::type_checker::get_type::get_type_with_hint;
use crate::type_checker::get_type::r#fn::{
    lift_or_left,
    with_constraint_lift_or_left
};
use crate::type_checker::get_type::r#type::GetTypeReturn;
use crate::{has_type, type_miss_match};

pub fn case_ri(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &MaybeType,
    assign_name: &str,
    assign_expr: &Expr,
    scope_expr: &Expr
) -> GetTypeReturn {
    // Hint scope_expr with expect_type and get scope_expr_type
    let scope_expr_type = get_type_with_hint(
        type_env,
        expr_env,
        scope_expr,
        expect_type
    );

    // 无需将 assign_name:assign_type 注入环境
    // 因为即使注入了环境也不能提供更多的类型信息(assign_type 是 Free)
    // 而且当找不到 assign_name 的类型信息时, env_ref case 会返回对 assign_name 的约束要求
    // 这种返回与注入 assign_name:Free 后再获取 assign_name 的类型信息时是一致的
    match scope_expr_type {
        // 返回了确定类型, 表明内层环境并没有用到 Let 的环境注入
        // 换言之, Let 表达式可以被直接简化为 scope_expr, 因为 scope_expr 与绑定无关
        // 如果改变实现, 也可在分析 assign_expr_type 的 L/ML 情况时得知这种无关性
        // 但这种实现会增加类型检查的复杂性, 应交由优化器实现
        Quad::L(scope_expr_type) => match lift_or_left(
            type_env,
            &scope_expr_type,
            expect_type
        ) {
            Some(t) => has_type!(t),
            None => type_miss_match!()
        },
        // 获取 scope_expr_type 时产生了约束
        Quad::ML(rc) => {
            let assign_type_constraint = rc
                .constraint
                .iter()
                .rev()
                .find(|(n, _)| n == assign_name)
                .map(|(_, t)| t);

            // 如果约束包含了 assign
            let constraint = if let Some(assign_type_constraint) =
                assign_type_constraint
            {
                // 获取确保限定成立的外层环境约束
                // 此时 assign_expr 无类型标注
                // 以旁路提供的 assign_type_constraint 为提示获取 assign_expr 的类型
                // 由于限定 assign_expr 为 assign_type_constraint 可能对外层环境产生约束
                // 需将这些约束传播以确保限定成立
                let outer_constraint = match get_type_with_hint(
                    type_env,
                    expr_env,
                    assign_expr,
                    //Hint assign_expr and get type of it
                    &assign_type_constraint
                        .clone()
                        .some()
                ) {
                    // 限定相容且未带来约束
                    Quad::L(_) => vec![],
                    // 限定相容且带来了约束, 传播之
                    Quad::ML(rc) => rc.constraint.clone(),
                    // 限定冲突或信息仍然不足, 推导失败
                    mr_r => return mr_r.clone()
                };

                // 将对 assign 的约束过滤掉, 并拼接起确保限定成立的外层约束作为最终约束
                rc.constraint
                    .iter()
                    .filter(|(n, _)| n != assign_name)
                    .chain(outer_constraint.iter())
                    .map(|x| x.clone())
                    .collect()
            } else {
                // 约束未包含 assign, 说明其全部作用于外层环境, 传播之
                rc.constraint
            };

            with_constraint_lift_or_left(
                constraint,
                type_env,
                &rc.r#type,
                expect_type
            )
        }
        // 旁路类型推导失败
        mr_r => mr_r.clone()
    }
}
