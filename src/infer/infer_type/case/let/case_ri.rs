use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::require_info::ReqInfo;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::option::OptionAnyExt;
use crate::infra::triple::Triple;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::OptType;

pub fn case_ri(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    require_info_ref_name: &str,
    constraint_acc: EnvRefConstraint,
    expect_type: &OptType,
    assign_name: &str,
    assign_expr: &Expr,
    scope_expr: &Expr
) -> InferTypeRet {
    // Hint scope_expr with expect_type and get scope_expr_type
    let scope_expr_type = scope_expr
        .with_opt_fallback_type(expect_type)
        .infer_type(type_env, expr_env);

    // 无需将 assign_name:assign_type 注入环境
    // 因为即使注入了环境也不能提供更多的类型信息(assign_type 是 Free)
    // 而且当找不到 assign_name 的类型信息时, namely case 会返回对 assign_name 的约束要求
    // 这种返回与注入 assign_name:Free 后再获取 assign_name 的类型信息时是一致的
    match scope_expr_type? {
        // 返回了确定类型, 表明内层环境并没有用到 assign 的环境注入
        // 换言之, Let 表达式可以被直接简化为 scope_expr, 因为 scope_expr 与绑定无关
        // 如果改变实现, 也可在分析 assign_expr_type 的 L/ML 情况时得知这种无关性
        // 但这种实现会增加类型检查的复杂性, 应交由优化器实现
        Triple::L(scope_expr_type) => match scope_expr_type
            .lift_to_or_left(type_env, expect_type)
        {
            // Some 和 None 分支的设计使得在此处编译器能够逐步提示代码错误
            // 从而有点编译器教人写代码的感觉(哈哈哈Rust)

            // scope_expr_type 在提升时出现了类型不相容, 优先返回该错误
            None => TypeMissMatch::of_type(
                &scope_expr_type,
                &expect_type.clone().unwrap()
            )
            .into(),

            // 由于 case_ri 分支仅当 assign 缺乏类型信息时才会进入
            // 因为 scope_expr 没有带来约束, 所以 assign 仍需类型信息
            // 改写或返回原错误, 改写是为了让无类型弃元错误正确地附加到 assign_name 上, 而不是被其他层级捕获
            _ => if require_info_ref_name == "_" {
                ReqInfo::of(assign_name, constraint_acc)
            } else {
                ReqInfo::of(require_info_ref_name, constraint_acc)
            }
            .into()
        },
        // 获取 scope_expr_type 时产生了约束
        Triple::M(rc) => {
            // 累积到外部约束
            let constraint_acc = match constraint_acc
                .extend_new(rc.constraint.clone())
            {
                Some(c) => c,
                None =>
                    return TypeMissMatch::of_constraint(
                        &constraint_acc,
                        &rc.constraint
                    )
                    .into(),
            };

            // 注入表达式环境
            // 新的环境可能包含对 assign_name 的约束和外层约束
            // 这些约束将有助于取得 assign_expr 的类型
            let new_expr_env =
                expr_env.extend_constraint_new(rc.constraint.clone());

            // 将 assign_name 约束到约束目标仍是必须的
            // 因为 assign_expr 可能不包含 assign_name
            let assign_type_constraint = rc
                .constraint
                .find(assign_name)
                .cloned();

            let constraint_acc = match assign_expr
                .with_opt_fallback_type(&assign_type_constraint)
                .infer_type(type_env, &new_expr_env)?
            {
                // 限定相容且未带来约束
                Triple::L(_) => constraint_acc,
                // 限定相容且带来了约束, 传播之
                Triple::M(rc) => match constraint_acc
                    .extend_new(rc.constraint.clone())
                {
                    Some(c) => c,
                    None =>
                        return TypeMissMatch::of_constraint(
                            &constraint_acc,
                            &rc.constraint
                        )
                        .into(),
                },
                // 仍然缺乏信息
                Triple::R(ri) => {
                    let constraint_acc =
                        constraint_acc.exclude_new(assign_name);

                    return if ri.ref_name == "_" {
                        // 拦截无类型弃元到 assign_name
                        ri.new_ref_name(assign_name)
                            .with_constraint_acc(constraint_acc)
                    } else {
                        ri.with_constraint_acc(constraint_acc)
                    };
                }
            };

            InferTypeRet::from_auto_lift(
                type_env,
                &rc.r#type,
                expect_type,
                // 将对 assign_name 的约束过滤掉
                // 因为 assign_expr 和 scope_expr 都有可能产生对 assign_name 的约束
                // 所以过滤要在最后进行, 这在上方获取 assign_expr_type 时的 MR 分支处也有体现
                constraint_acc
                    .exclude_new(assign_name)
                    .some()
            )
        }

        Triple::R(ri) => ri.with_constraint_acc(constraint_acc)
    }
}
