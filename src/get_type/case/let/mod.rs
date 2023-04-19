mod case_ri;
mod case_t_rc;

use crate::empty_constraint;
use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::get_type::case::r#let::case_ri::case_ri;
use crate::get_type::case::r#let::case_t_rc::case_t_rc;
use crate::get_type::get_type_with_hint;
use crate::get_type::r#type::GetTypeReturn;
use crate::infra::alias::MaybeType;
use crate::infra::quad::Quad;
use crate::parser::expr::r#type::Expr;

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &MaybeType,
    assign_name: &String,
    assign_type: &MaybeType,
    assign_expr: &Expr,
    scope_expr: &Expr
) -> GetTypeReturn {
    // Hint assign_expr with assign_type and get assign_expr_type
    let assign_expr_type = get_type_with_hint(
        type_env,
        expr_env,
        assign_expr,
        assign_type
    );

    match assign_expr_type {
        // 在获取 assign_expr_type 时产生了约束, 这些约束一定作用于外层环境, 传播之
        // 这种传播可能是一种约束传播, 在 assign_expr 无类型而 assign_type 存在的情况下
        // assign_type 会对 assign_expr 产生类型限定(通过 hint), 这使得约束从内层传播到了外层
        // L 与 ML 的唯一区别是 ML 额外携带了一些对外层环境的约束, 需要传播这些约束
        Quad::L(_) | Quad::ML(_) => {
            let (assign_expr_type, constraint_acc) = match assign_expr_type {
                Quad::L(t) => (t, empty_constraint!()),
                // 需传播额外携带的约束
                Quad::ML(rc) => (rc.r#type, rc.constraint),
                _ => panic!("Impossible assign_expr_type: {assign_type:?}")
            };

            case_t_rc(
                type_env,
                &expr_env,
                assign_expr_type,
                constraint_acc,
                expect_type,
                assign_name,
                assign_type,
                assign_expr,
                scope_expr
            )
        }

        // 获取 assign_expr_type 时信息不足, 而 scope_expr 中可能存在对 assign_name 的类型约束
        // 这种约束可能对获取 assign_expr_type 有所帮助, 所以可以启用旁路类型推导
        // 旁路类型推导仅在外层信息未知时适用, 因为如果外层信息已知
        // 那么外层信息将具备更高的优先级, 此时使用类型旁路会让内层类型超越外层约束的限制
        // 所以在此处, 仅当 assign_type 和 assign_expr 均无类型信息时, 才能启用旁路类型推导
        Quad::MR(require_info)
            if assign_type.is_none() &&
                assign_expr.is_no_type_annot() =>
            case_ri(
                type_env,
                &expr_env,
                require_info,
                expect_type,
                assign_name,
                assign_expr,
                scope_expr
            ),

        mr_r => mr_r
    }
}
