use std::rc::Rc;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::option::OptionAnyExt;
use crate::infra::triple::Triple;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::OptType;
use crate::parser::r#type::r#type::Type;

pub fn case_t_rc<F>(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    lhs_input_type: Type,
    lhs_output_type: Type,
    expect_type: &OptType,
    rhs_expr: &Expr,
    typed_expr_cons: F
) -> InferTypeRet
where
    F: Fn(Type, Expr) -> Expr
{
    match rhs_expr
        .with_fallback_type(&lhs_input_type)
        .infer_type(type_env, expr_env)?
    {
        result @ (Triple::L(_) | Triple::M(_)) => {
            let (typed_rhs_expr, constraint) =
                result.unwrap_expr_constraint();

            let rhs_expr_type = typed_rhs_expr.unwrap_type_annot();

            // 验证输入的类型相容性
            if rhs_expr_type.can_lift_to(type_env, &lhs_input_type) {
                // 验证输出的类型相容性
                InferTypeRet::from_auto_lift(
                    type_env,
                    &lhs_output_type,
                    // Apply 的期望类型也是 lhs_expr 的期望输出类型
                    expect_type,
                    constraint.some(),
                    |t| typed_expr_cons(t, typed_rhs_expr.clone())
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
