mod case_ri;
mod case_t_rc;

use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::get_type::case::cond::case_ri::case_ri;
use crate::get_type::case::cond::case_t_rc::case_t_rc;
use crate::get_type::get_type_with_hint;
use crate::get_type::r#type::type_miss_match::TypeMissMatch;
use crate::get_type::r#type::GetTypeReturn;
use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::parser::expr::r#type::Expr;
use crate::unify::can_lift;
use crate::{bool_type, empty_constraint};

// TODO: 外部环境约束同层传播完备性
pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &MaybeType,
    bool_expr: &Expr,
    then_expr: &Expr,
    else_expr: &Expr
) -> GetTypeReturn {
    let bool_expr_type = get_type_with_hint(
        type_env,
        expr_env,
        bool_expr,
        &bool_type!().some()
    );

    // bool_expr must be boolean types
    let constraint_acc = match &bool_expr_type {
        Quad::L(bool_expr_type) =>
            if can_lift(type_env, &bool_expr_type, &bool_type!()) {
                empty_constraint!()
            } else {
                return TypeMissMatch::of_type(
                    bool_expr_type,
                    &bool_type!()
                )
                .into();
            },
        Quad::ML(rc) =>
            if can_lift(type_env, &rc.r#type, &bool_type!()) {
                rc.constraint.clone()
            } else {
                return TypeMissMatch::of_type(
                    &rc.r#type,
                    &bool_type!()
                )
                .into();
            },
        // 需要类型信息或者类型不匹配, 由于 Cond 没有环境注入, 不应处理这些情况
        mr_r => return mr_r.clone()
    };

    // TODO: 相似用例检查
    // 由于求 bool_expr_type 产生的约束可能对接下来有帮助, 所以需要注入到环境
    let expr_env =
        &expr_env.extend_constraint_new(constraint_acc.clone());

    let then_expr_type = get_type_with_hint(
        type_env,
        expr_env,
        then_expr,
        expect_type
    );

    match then_expr_type {
        Quad::L(_) | Quad::ML(_) => {
            let (then_expr_type, constraint_acc) =
                match then_expr_type {
                    Quad::L(then_expr_type) =>
                        (then_expr_type, constraint_acc),
                    Quad::ML(rc) => match constraint_acc
                        .extend_new(rc.constraint.clone())
                    {
                        Some(constraint) => (rc.r#type, constraint),
                        None =>
                            return TypeMissMatch::of_constraint(
                                &constraint_acc,
                                &rc.constraint
                            ).into()
                    },
                    _ => panic!(
                        "Impossible then_expr_type: {then_expr_type:?}"
                    )
                };

            // TODO: 相似用例检查
            // 与上同理
            let expr_env = &expr_env
                .extend_constraint_new(constraint_acc.clone());

            case_t_rc(
                type_env,
                expr_env,
                then_expr_type,
                constraint_acc,
                expect_type,
                else_expr
            )
        }

        Quad::MR(_)
            if then_expr.is_no_type_annot() &&
                expect_type.is_none() =>
            case_ri(
                type_env, expr_env, bool_expr, else_expr, then_expr
            ),

        mr_r => mr_r
    }
}
