use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::get_type::get_type_with_hint;
use crate::get_type::r#fn::with_constraint_lift_or_left;
use crate::get_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::get_type::r#type::type_miss_match::TypeMissMatch;
use crate::get_type::r#type::GetTypeReturn;
use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::Type;
use crate::unify::can_lift;

pub fn case_t_rc(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    lhs_input_type: Type,
    lhs_output_type: Type,
    constraint_acc: EnvRefConstraint,
    expect_type: &MaybeType,
    rhs_expr: &Expr
) -> GetTypeReturn {
    // Apply 的期望类型也是 lhs_expr 的期望输出类型
    let expect_output_type = expect_type;

    match get_type_with_hint(
        type_env,
        expr_env,
        rhs_expr,
        &lhs_input_type.clone().some()
    ) {
        Quad::L(rhs_expr_type) => {
            // 验证输入的类型相容性
            if can_lift(type_env, &rhs_expr_type, &lhs_input_type) {
                // 验证输出的类型相容性
                with_constraint_lift_or_left(
                    constraint_acc,
                    type_env,
                    &lhs_output_type,
                    expect_output_type
                )
            } else {
                TypeMissMatch::of_type(
                    &rhs_expr_type,
                    &lhs_input_type
                )
                .into()
            }
        }
        Quad::ML(rc) => {
            // 输入类型相容且约束相容
            if can_lift(type_env, &rc.r#type, &lhs_input_type) {
                if let Some(constraint) =
                    constraint_acc.extend_new(rc.constraint.clone())
                {
                    with_constraint_lift_or_left(
                        constraint,
                        type_env,
                        &lhs_output_type,
                        expect_output_type
                    )
                } else {
                    TypeMissMatch::of_constraint(
                        &constraint_acc,
                        &rc.constraint
                    )
                    .into()
                }
            } else {
                TypeMissMatch::of_type(&rc.r#type, &lhs_input_type)
                    .into()
            }
        }
        mr_r => mr_r
    }
}
