use std::ops::Deref;

use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::parser::expr::Expr;
use crate::parser::r#type::Type;
use crate::type_checker::env::expr_env::ExprEnv;
use crate::type_checker::env::type_env::TypeEnv;
use crate::type_checker::get_type::get_type_with_hint;
use crate::type_checker::get_type::r#fn::with_constraint_lift_or_left;
use crate::type_checker::get_type::r#type::GetTypeReturn;
use crate::type_miss_match;
use crate::unifier::can_lift;

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
        Quad::L(lhs_type) => (lhs_type, vec![]),
        Quad::ML(rc) => (rc.r#type, rc.constraint),
        _ => panic!("Impossible lhs_expr_type: {:?}", lhs_expr_type)
    };

    if let Type::ClosureType(input_type, output_type) = lhs_expr_type
    {
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
                if can_lift(type_env, &rhs_expr_type, &input_type) {
                    // 验证输出的类型相容性
                    with_constraint_lift_or_left(
                        constraint,
                        type_env,
                        &output_type,
                        expect_output_type
                    )
                } else {
                    type_miss_match!()
                }
            }
            Quad::ML(rc) => {
                if can_lift(type_env, &rc.r#type, &input_type) {
                    with_constraint_lift_or_left(
                        vec![constraint, rc.constraint].concat(),
                        type_env,
                        &output_type,
                        expect_output_type
                    )
                } else {
                    type_miss_match!()
                }
            }
            mr_r => mr_r
        }
    } else {
        // lhs_expr_type must be ClosureType
        type_miss_match!()
    }
}
