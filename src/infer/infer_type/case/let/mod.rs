mod case_ri;
mod case_t_rc;
#[cfg(test)]
mod test;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::case::r#let::case_ri::case_ri;
use crate::infer::infer_type::case::r#let::case_t_rc::case_t_rc;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::option::OptionAnyExt;
use crate::infra::r#box::BoxAnyExt;
use crate::infra::triple::Triple;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::OptType;

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &OptType,
    assign_name: &String,
    assign_type: &OptType,
    assign_expr: &Expr,
    scope_expr: &Expr
) -> InferTypeRet {
    // Hint assign_expr with assign_type and get assign_expr_type
    match assign_expr
        .with_opt_fallback_type(assign_type)
        .infer_type(type_env, expr_env)?
    {
        // 在获取 assign_expr_type 时产生了约束, 这些约束一定作用于外层环境, 传播之
        // 这种传播可能是一种约束传播, 在 assign_expr 无类型而 assign_type 存在的情况下
        // assign_type 会对 assign_expr 产生类型限定(通过 hint), 这使得约束从内层传播到了外层
        // L 与 ML 的唯一区别是 ML 额外携带了一些对外层环境的约束, 需要传播这些约束
        result @ (Triple::L(_) | Triple::M(_)) => {
            let (typed_assign_expr, constraint) =
                result.unwrap_expr_constraint();

            // 过滤掉对 assign_name 的约束(对于 ML
            let constraint_acc =
                constraint.exclude_new(assign_name.as_str());

            let assign_expr_type =
                typed_assign_expr.unwrap_type_annot();

            // Lift assign_expr_type to assign_type
            let assign_type = match assign_expr_type
                .lift_to_or_left(type_env, assign_type)
            {
                None =>
                    return TypeMissMatch::of_type(
                        &assign_expr_type,
                        &assign_type.clone().unwrap()
                    )
                    .into(),
                Some(t) => t
            };

            // Inject constraints to env
            let new_expr_env = expr_env
                .extend_constraint_new(constraint_acc.clone());

            // Inject assign to env
            let new_expr_env = new_expr_env.extend_new(
                assign_name.to_string(),
                assign_type.clone().some(),
                typed_assign_expr
                    .clone()
                    .some()
            );

            case_t_rc(
                type_env,
                &new_expr_env,
                expect_type,
                scope_expr,
                |type_annot, typed_scope_expr| {
                    Expr::Let(
                        type_annot.some(),
                        assign_name.to_string(),
                        assign_type.clone().some(),
                        typed_assign_expr
                            .clone()
                            .boxed(),
                        typed_scope_expr.boxed()
                    )
                }
            )?
            .with_constraint_acc(constraint_acc)
        }

        // 获取 assign_expr_type 时信息不足, 而 scope_expr 中可能存在对 assign_name 的类型约束
        // 这种约束可能对获取 assign_expr_type 有所帮助, 所以可以启用旁路类型推导
        // 旁路类型推导仅在外层信息未知时适用, 因为如果外层信息已知
        // 那么外层信息将具备更高的优先级, 此时使用类型旁路会让内层类型超越外层约束的限制
        // 所以在此处, 仅当 assign_type 和 assign_expr 均无类型信息时, 才能启用旁路类型推导
        Triple::R(ri)
            if assign_type.is_none() &&
                assign_expr.is_no_type_annot() =>
        {
            let new_expr_env =
                expr_env.extend_constraint_new(ri.constraint.clone());

            case_ri(
                type_env,
                &new_expr_env,
                &ri.ref_name,
                expect_type,
                assign_name,
                assign_expr,
                scope_expr
            )?
            .with_constraint_acc(ri.constraint)?
            // 将对 assign_name 的约束过滤掉
            // 因为 assign_expr 和 scope_expr 都有可能产生对 assign_name 的约束
            .exclude_constraint(assign_name.as_str())
        }

        ri => ri.into()
    }
}
