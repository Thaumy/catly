use std::ops::Deref;

use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::get_type::get_type_with_hint;
use crate::get_type::r#fn::with_constraint_lift_or_left;
use crate::get_type::r#type::GetTypeReturn;
use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::Type;
use crate::unify::can_lift;
use crate::{empty_constraint, type_miss_match};

pub fn case_t_rc(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    lhs_expr_type: GetTypeReturn,
    expect_type: &MaybeType,
    rhs_expr: &Expr
) -> GetTypeReturn {
    // Apply 的期望类型也是 lhs_expr 的期望输出类型
    let expect_output_type = expect_type;

    let (lhs_expr_type, constraint) = match lhs_expr_type {
        Quad::L(lhs_type) => (lhs_type, empty_constraint!()),
        Quad::ML(rc) => (rc.r#type, rc.constraint),
        _ => panic!("Impossible lhs_expr_type: {lhs_expr_type:?}")
    };

    match lhs_expr_type {
        Type::ClosureType(input_type, output_type) => {
            match get_type_with_hint(
                type_env,
                expr_env,
                rhs_expr,
                &input_type
                    .deref()
                    .clone()
                    .some()
            ) {
                Quad::L(rhs_expr_type) => {
                    // 验证输入的类型相容性
                    if can_lift(type_env, &rhs_expr_type, &input_type)
                    {
                        // 验证输出的类型相容性
                        with_constraint_lift_or_left(
                            constraint,
                            type_env,
                            &output_type,
                            expect_output_type
                        )
                    } else {
                        type_miss_match!(format!(
                            "{rhs_expr_type:?} <> {input_type:?}"
                        ))
                    }
                }
                Quad::ML(rc) => {
                    // 输入类型相容且约束相容
                    if can_lift(type_env, &rc.r#type, &input_type) &&
                        let Some(constraint) = rc
                            .constraint
                            .extend_new(constraint)
                    {
                        with_constraint_lift_or_left(
                            constraint,
                            type_env,
                            &output_type,
                            expect_output_type,
                        )
                    } else {
                        type_miss_match!(format!("{:?} <> {input_type:?}", rc.r#type))
                    }
                }
                mr_r => mr_r
            }
        }
        // lhs_expr_type must be ClosureType, PartialClosureType is used for hint only
        _ => type_miss_match!(format!(
            "{lhs_expr_type:?} <> ClosureType"
        ))
    }
}
