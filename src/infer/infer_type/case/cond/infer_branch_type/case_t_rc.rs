use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
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
    then_expr_type: Type,
    expect_type: &OptType,
    else_expr: &Expr
) -> InferTypeRet {
    // 当 expect_type 无类型时, 使用 then_expr_type hint
    let expect_type = match expect_type {
        Some(expect_type) =>
            if then_expr_type.can_lift_to(type_env, &expect_type) {
                expect_type.clone()
            } else {
                return TypeMissMatch::of_type(
                    &then_expr_type,
                    &expect_type
                )
                .into();
            },
        None => then_expr_type.clone()
    };

    match else_expr
        .with_fallback_type(&expect_type)
        .infer_type(type_env, expr_env)?
    {
        else_expr_type @ (Triple::L(_) | Triple::M(_)) => {
            let (else_expr_type, constraint_acc) =
                else_expr_type.unwrap_type_constraint();

            InferTypeRet::from_auto_lift(
                type_env,
                &else_expr_type,
                &expect_type.some(),
                constraint_acc.some()
            )
        }

        Triple::R(ri) => ri.into()
    }
}
