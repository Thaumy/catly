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
    assign_expr_type: Type,
    expect_type: &OptType,
    assign_name: &str,
    assign_type: &OptType,
    assign_expr: &Expr,
    scope_expr: &Expr,
    typed_assign_expr: Expr
) -> InferTypeRet {
    // Lift assign_expr_type to assign_type
    // TODO: lift out this
    // TODO: 相似用例检查
    let assign_type = match assign_expr_type
        .lift_to_or_left(type_env, assign_type)
    {
        None =>
            return TypeMissMatch::of_type(
                &assign_expr_type,
                &assign_type.clone().unwrap()
            )
            .into(),
        Some(t) => t
    };

    // Env inject
    let expr_env = expr_env.extend_new(
        assign_name.to_string(),
        assign_type.clone().some(),
        assign_expr.clone().some()
    );

    // Hint scope_expr with expect_type and get scope_expr_type
    match scope_expr
        .with_opt_fallback_type(expect_type)
        .infer_type(type_env, &expr_env)?
    {
        scope_expr_type @ (Triple::L(_) | Triple::M(_)) => {
            let (scope_expr_type, constraint, typed_scope_expr) =
                scope_expr_type.unwrap_type_constraint_expr();
            InferTypeRet::from_auto_lift(
                type_env,
                &scope_expr_type,
                expect_type,
                constraint.some(),
                |t| {
                    Expr::Let(
                        t.some(),
                        assign_name.to_string(),
                        assign_type.clone().some(),
                        typed_assign_expr
                            .clone()
                            .boxed(),
                        typed_scope_expr
                            .clone()
                            .boxed()
                    )
                }
            )
        }

        Triple::R(ri) => ri.into()
    }
}
