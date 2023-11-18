mod infer_branch_type;
#[cfg(test)]
mod test;

use std::rc::Rc;

use crate::infer::env::bool_type;
use crate::infer::env::ExprEnv;
use crate::infer::env::TypeEnv;
use crate::infer::infer_type::case::cond::infer_branch_type::infer_branch_type;
use crate::infer::infer_type::InferTypeRet;
use crate::infer::infer_type::ReqConstraint;
use crate::infer::infer_type::ReqInfo;
use crate::infer::infer_type::TypeMissMatch;
use crate::infra::Triple;
use crate::infra::WrapQuad;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::OptType;

pub fn case(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    expect_type: &OptType,
    bool_expr: &Expr,
    then_expr: &Expr,
    else_expr: &Expr
) -> InferTypeRet {
    match bool_expr
        .with_fallback_type(&bool_type!())
        .infer_type(type_env, expr_env)?
    {
        result @ (Triple::L(_) | Triple::M(_)) => {
            let (typed_bool_expr, constraint) =
                result.unwrap_expr_constraint();

            let bool_expr_type = typed_bool_expr.unwrap_type_annot();

            if bool_expr_type.can_lift_to(type_env, &bool_type!()) {
                // 由于求 bool_expr_type 产生的约束可能对接下来有帮助, 所以需要注入到环境
                let new_expr_env = &expr_env
                    .extend_constraint_new(constraint.clone());

                infer_branch_type(
                    type_env,
                    new_expr_env,
                    expect_type,
                    typed_bool_expr,
                    then_expr,
                    else_expr
                )?
                .with_constraint_acc(constraint)
            } else {
                // bool_expr must be boolean types
                TypeMissMatch::of_type(bool_expr_type, &bool_type!())
                    .into()
            }
        }
        // 求取分支类型, 因为分支约束可能有助于求得 bool_expr 类型
        // 约束将在下一轮次被注入环境, 同时也会再次求 bool_expr 类型
        Triple::R(ri) => {
            let constraint_acc = ri.constraint.clone();

            let new_expr_env = &expr_env
                .extend_constraint_new(constraint_acc.clone());

            match infer_branch_type(
                type_env,
                new_expr_env,
                expect_type,
                // 因为此推导的目的是收集依赖, 不会使用最终产生的结果
                // 所以使用不完备类型信息的 bool_expr 是可以的(其实就是因为必须要传参
                bool_expr.clone(),
                then_expr,
                else_expr
            )? {
                // 产生约束, 改写错误以便下一轮对 bool_expr 进行类型获取
                Triple::M(ReqConstraint { constraint, .. }) =>
                    ReqInfo::of(ri.ref_name, constraint).into(),
                // 未产生约束, 返回原错误
                Triple::L(_) => ri.wrap_quad_mr(),
                // 分支表达式也无非获取类型, 由于约束已经累积, 传播之
                r => r.into()
            }?
            .with_constraint_acc(constraint_acc)
        }
    }
}
