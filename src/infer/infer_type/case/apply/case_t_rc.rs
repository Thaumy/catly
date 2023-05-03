use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::option::OptionAnyExt;
use crate::infra::r#box::BoxAnyExt;
use crate::infra::triple::Triple;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::OptType;
use crate::parser::r#type::r#type::Type;

pub fn case_t_rc(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    lhs_input_type: Type,
    lhs_output_type: Type,
    expect_type: &OptType,
    rhs_expr: &Expr,
    typed_lhs_expr: Expr
) -> InferTypeRet {
    // Apply 的期望类型也是 lhs_expr 的期望输出类型
    let expect_output_type = expect_type;

    match rhs_expr
        .with_fallback_type(&lhs_input_type)
        .infer_type(type_env, expr_env)?
    {
        rhs_expr_type @ (Triple::L(_) | Triple::M(_)) => {
            let (rhs_expr_type, constraint, typed_rhs_expr) =
                rhs_expr_type.unwrap_type_constraint_expr();

            // 验证输入的类型相容性
            if rhs_expr_type.can_lift_to(type_env, &lhs_input_type) {
                // 验证输出的类型相容性
                InferTypeRet::from_auto_lift(
                    type_env,
                    &lhs_output_type,
                    expect_output_type,
                    constraint.some(),
                    |t| {
                        Expr::Apply(
                            t.some(),
                            typed_lhs_expr.clone().boxed(),
                            typed_rhs_expr.clone().boxed()
                        )
                    }
                )
            } else {
                TypeMissMatch::of_type(
                    &rhs_expr_type,
                    &lhs_input_type
                )
                .into()
            }
        }

        Triple::R(ri) => ri.into()
    }
}
