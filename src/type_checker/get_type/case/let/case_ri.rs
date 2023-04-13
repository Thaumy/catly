use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::parser::expr::Expr;
use crate::type_checker::get_type::r#fn::{
    lift_or_left,
    with_constraint_lift_or_left
};
use crate::type_checker::get_type::r#type::{
    EnvRefConstraint,
    GetTypeReturn,
    RequireInfo
};
use crate::type_checker::get_type::{get_type, get_type_with_hint};
use crate::{
    has_type,
    require_constraint,
    require_info,
    type_miss_match
};

pub fn case_ri(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    require_info: RequireInfo,
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
        // 返回了确定类型, 表明内层环境并没有用到 assign 的环境注入
        // 换言之, Let 表达式可以被直接简化为 scope_expr, 因为 scope_expr 与绑定无关
        // 如果改变实现, 也可在分析 assign_expr_type 的 L/ML 情况时得知这种无关性
        // 但这种实现会增加类型检查的复杂性, 应交由优化器实现
        Quad::L(scope_expr_type) => match lift_or_left(
            type_env,
            &scope_expr_type,
            expect_type
        ) {
            // Some 和 None 分支的设计使得在此处编译器能够逐步提示代码错误
            // 从而有点编译器教人写代码的感觉(哈哈哈Rust)

            // scope_expr_type 在提升时出现了类型不相容, 优先返回该错误
            None => type_miss_match!(),

            // 由于 case_ri 分支仅当 assign 缺乏类型信息时才会进入
            // 因为 scope_expr 没有带来约束, 所以 assign 仍需类型信息
            // 改写或返回原错误, 改写是为了让无类型弃元错误正确的附加到 assign_name 上, 而不是被其他层级捕获
            _ =>
                if require_info.ref_name == "_" {
                    require_info!(assign_name.to_string())
                } else {
                    Quad::MR(require_info)
                },
        },
        // 获取 scope_expr_type 时产生了约束
        Quad::ML(rc) => {
            let assign_type_constraint = rc
                .constraint
                .find(assign_name);

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
                    Quad::L(_) => EnvRefConstraint::empty(),
                    // 限定相容且带来了约束, 传播之
                    Quad::ML(rc) => rc.constraint.clone(),
                    // 限定冲突或信息仍然不足, 推导失败
                    mr_r => return mr_r.clone()
                };

                // 将对 assign 的约束过滤掉, 并拼接起确保限定成立的外层约束作为最终约束
                match rc
                    .constraint
                    .filter_new(|(n, _)| n != assign_name)
                    .extend_new(outer_constraint)
                {
                    Some(constraint) => constraint,
                    None => return type_miss_match!()
                }
            } else {
                // 约束不包含 assign, 关于此处实现的讨论可参见上方的 L 分支
                return match lift_or_left(
                    type_env,
                    &rc.r#type,
                    expect_type
                ) {
                    None => type_miss_match!(),
                    _ =>
                        if require_info.ref_name == "_" {
                            require_info!(assign_name.to_string())
                        } else {
                            Quad::MR(require_info)
                        },
                };
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
