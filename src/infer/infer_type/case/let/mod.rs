mod case_ri;
mod case_t_rc;

#[cfg(test)]
mod test;

use std::rc::Rc;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::case::r#let::case_ri::case_ri;
use crate::infer::infer_type::case::r#let::case_t_rc::case_t_rc;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::option::WrapOption;
use crate::infra::rc::RcAnyExt;
use crate::infra::triple::Triple;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::OptType;

pub fn case(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    expect_type: &OptType,
    rec_assign: &bool,
    assign_name: &String,
    assign_type: &OptType,
    assign_expr: &Expr,
    scope_expr: &Expr
) -> InferTypeRet {
    let expr_env = if *rec_assign {
        // 如果 Let 是递归绑定的, 那么需要把名和类型注入环境
        expr_env.extend_new(
            assign_name,
            // 在这里可能注入无类型约束, 但这是合理的, 因为如果 Let 是递归绑定的
            // 那么外层环境中出现的某个与 assign_name 同名的绑定将与 assign_expr 无关
            assign_type.clone(),
            None
        )
    } else {
        expr_env.clone()
    };

    // Hint assign_expr with assign_type and get assign_expr_type
    match assign_expr
        .with_opt_fallback_type(assign_type)
        .infer_type(type_env, &expr_env)?
    {
        // 在获取 assign_expr 类型时产生了约束
        // L 与 ML 的唯一区别是 ML 额外携带了一些对外层环境的约束, 需要传播这些约束
        result @ (Triple::L(_) | Triple::M(_)) => {
            let (typed_assign_expr, constraint) =
                result.unwrap_expr_constraint();

            let constraint_acc = if *rec_assign {
                // 如果 Let 是递归的, 则应去除约束中针对 assign_name 的约束, 因为它与外层环境无关
                constraint.exclude_new(assign_name.as_str())
            } else {
                // 如果 Let 是非递归的, 那么约束全部作用于外层环境
                constraint
            };

            let assign_expr_type =
                typed_assign_expr.unwrap_type_annot();

            // Lift assign_expr_type to assign_type
            let assign_type = match assign_expr_type
                .lift_to_or_left(type_env, assign_type)
            {
                None =>
                    return TypeMissMatch::of_type(
                        assign_expr_type,
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
                assign_type
                    .clone()
                    .wrap_some(),
                typed_assign_expr
                    .clone()
                    .wrap_some()
            );

            case_t_rc(
                type_env,
                &new_expr_env,
                expect_type,
                scope_expr,
                |type_annot, typed_scope_expr| {
                    Expr::Let(
                        type_annot.wrap_some(),
                        *rec_assign,
                        assign_name.to_string(),
                        assign_type
                            .clone()
                            .wrap_some(),
                        typed_assign_expr.clone().rc(),
                        typed_scope_expr.rc()
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

            let result = case_ri(
                type_env,
                &new_expr_env,
                &ri.ref_name,
                expect_type,
                rec_assign,
                assign_name,
                assign_expr,
                scope_expr
            )?
            .with_constraint_acc(ri.constraint)?;

            if *rec_assign {
                // 如果 Let 是递归的, 那么应过滤掉所有对于 assign_name 的约束
                result.exclude_constraint(assign_name.as_str())
            } else {
                // 如果 Let 是非递归的, 那么应保留对于 assign_name 的约束, 因为它作用于外层
                // 由于 scope_expr 产生的 assign_name 约束已被过滤, 此处的 assign_name 约束完全由 assign_expr 产生
                result.into()
            }
        }

        ri => ri.into()
    }
}
