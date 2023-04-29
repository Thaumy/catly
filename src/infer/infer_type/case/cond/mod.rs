mod case_ri;
mod case_t_rc;
#[cfg(test)]
mod test;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::r#macro::bool_type;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::case::cond::case_ri::case_ri;
use crate::infer::infer_type::case::cond::case_t_rc::case_t_rc;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::quad::Quad;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::OptType;

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &OptType,
    bool_expr: &Expr,
    then_expr: &Expr,
    else_expr: &Expr
) -> InferTypeRet {
    let bool_expr_type = bool_expr
        .with_fallback_type(&bool_type!())
        .infer_type(type_env, expr_env);

    // bool_expr must be boolean types
    let constraint_acc = match &bool_expr_type {
        Quad::L(bool_expr_type) =>
            if bool_expr_type.can_lift_to(type_env, &bool_type!()) {
                EnvRefConstraint::empty()
            } else {
                return TypeMissMatch::of_type(
                    bool_expr_type,
                    &bool_type!()
                )
                .into();
            },
        Quad::ML(rc) =>
            if rc
                .r#type
                .can_lift_to(type_env, &bool_type!())
            {
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

    // 由于求 bool_expr_type 产生的约束可能对接下来有帮助, 所以需要注入到环境
    let expr_env =
        &expr_env.extend_constraint_new(constraint_acc.clone());

    let then_expr_type = then_expr
        .with_opt_fallback_type(expect_type)
        .infer_type(type_env, expr_env);

    match then_expr_type {
        Quad::L(_) | Quad::ML(_) => {
            let (then_expr_type, constraint) =
                then_expr_type.unwrap_type_and_constraint();
            let constraint_acc =
                match constraint_acc.extend_new(constraint.clone()) {
                    Some(constraint) => constraint,
                    None =>
                        return TypeMissMatch::of_constraint(
                            &constraint_acc,
                            &constraint
                        )
                        .into(),
                };

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
