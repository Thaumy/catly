mod infer_branch_type;
/*#[cfg(test)]
mod test;
*/
use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::r#macro::bool_type;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::case::cond::infer_branch_type::infer_branch_type;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::require_constraint::ReqConstraint;
use crate::infer::infer_type::r#type::require_info::ReqInfo;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::quad::QuadAnyExt;
use crate::infra::triple::Triple;
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
        .infer_type(type_env, expr_env)?;

    // bool_expr must be boolean types
    let constraint_acc = match &bool_expr_type {
        Triple::L(bool_expr_type) =>
            if bool_expr_type.can_lift_to(type_env, &bool_type!()) {
                EnvRefConstraint::empty()
            } else {
                return TypeMissMatch::of_type(
                    bool_expr_type,
                    &bool_type!()
                )
                .into();
            },
        Triple::M(rc) =>
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
        // 求取分支类型, 因为分支约束可能有助于求得 bool_expr 类型
        // 约束将在下一轮次被注入环境, 同时也会再次求 bool_expr 类型
        Triple::R(ri) => {
            let constraint_acc = ri.constraint.clone();

            let new_expr_env = &expr_env
                .extend_constraint_new(constraint_acc.clone());

            return match infer_branch_type(
                type_env,
                new_expr_env,
                expect_type,
                bool_expr,
                then_expr,
                else_expr
            )? {
                // 产生约束, 改写错误以便下一轮对 bool_expr 进行类型获取
                Triple::M(ReqConstraint { constraint, .. }) =>
                    ReqInfo::of(ri.ref_name.clone(), constraint)
                        .into(),
                // 未产生约束, 返回原错误
                Triple::L(_) => ri.clone().quad_mr(),
                // 分支表达式也无非获取类型, 由于约束已经累积, 传播之
                r => r.into()
            }?
            .with_constraint_acc(constraint_acc);
        }
    };

    // 由于求 bool_expr_type 产生的约束可能对接下来有帮助, 所以需要注入到环境
    let new_expr_env =
        &expr_env.extend_constraint_new(constraint_acc.clone());

    infer_branch_type(
        type_env,
        new_expr_env,
        expect_type,
        bool_expr,
        then_expr,
        else_expr
    )?
    .with_constraint_acc(constraint_acc)
}
