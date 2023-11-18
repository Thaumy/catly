use std::rc::Rc;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::InferTypeRet;
use crate::infer::infer_type::TypeMissMatch;
use crate::infra::Triple;
use crate::infra::WrapOption;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::OptType;
use crate::parser::r#type::Type;

pub fn case_t_rc<F>(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    then_expr_type: Type,
    expect_type: &OptType,
    else_expr: &Expr,
    typed_expr_cons: F
) -> InferTypeRet
where
    F: Fn(Type, Expr) -> Expr
{
    // 当 expect_type 无类型时, 使用 then_expr_type hint
    let expect_type = match expect_type {
        Some(expect_type) =>
            if then_expr_type.can_lift_to(type_env, expect_type) {
                expect_type.clone()
            } else {
                return TypeMissMatch::of_type(
                    &then_expr_type,
                    expect_type
                )
                .into();
            },
        None => then_expr_type
    };

    match else_expr
        .with_fallback_type(&expect_type)
        .infer_type(type_env, expr_env)?
    {
        result @ (Triple::L(_) | Triple::M(_)) => {
            let (typed_else_expr, constraint_acc) =
                result.unwrap_expr_constraint();

            InferTypeRet::from_auto_lift(
                type_env,
                typed_else_expr.unwrap_type_annot(),
                &expect_type.wrap_some(),
                constraint_acc.wrap_some(),
                |t| typed_expr_cons(t, typed_else_expr.clone())
            )
        }

        Triple::R(ri) => ri.into()
    }
}
