use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::option::OptionAnyExt;
use crate::infra::triple::Triple;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::OptType;
use crate::parser::r#type::r#type::Type;

pub fn case_t_rc(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    lhs_input_type: Type,
    lhs_output_type: Type,
    constraint_acc: EnvRefConstraint,
    expect_type: &OptType,
    rhs_expr: &Expr
) -> InferTypeRet {
    // Apply 的期望类型也是 lhs_expr 的期望输出类型
    let expect_output_type = expect_type;

    match rhs_expr
        .with_fallback_type(&lhs_input_type)
        .infer_type(type_env, expr_env)?
    {
        rhs_expr_type @ (Triple::L(_) | Triple::M(_)) => {
            let (rhs_expr_type, constraint) =
                rhs_expr_type.unwrap_type_constraint();

            let constraint_acc =
                match constraint_acc.extend_new(constraint.clone()) {
                    Some(c) => c,
                    // 约束不相容
                    // 同样, 由于 lhs_expr_type 的约束注入, 理论上这个分支也不会执行
                    // 保留是处于保险考虑
                    None =>
                        return TypeMissMatch::of_constraint(
                            &constraint_acc,
                            &constraint
                        )
                        .into(),
                };

            // 验证输入的类型相容性
            if rhs_expr_type.can_lift_to(type_env, &lhs_input_type) {
                // 验证输出的类型相容性
                InferTypeRet::from_auto_lift(
                    type_env,
                    &lhs_output_type,
                    expect_output_type,
                    constraint_acc.some()
                )
            } else {
                TypeMissMatch::of_type(
                    &rhs_expr_type,
                    &lhs_input_type
                )
                .into()
            }
        }

        Triple::R(ri) => ri.with_constraint_acc(constraint_acc)
    }
}
