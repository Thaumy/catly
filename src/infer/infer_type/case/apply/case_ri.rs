use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infra::r#box::BoxAnyExt;
use crate::infra::triple::Triple;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::OptType;
use crate::parser::r#type::r#type::Type;

pub fn case_ri(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &OptType,
    lhs_expr: &Expr,
    rhs_expr: &Expr
) -> InferTypeRet {
    // 尝试从 rhs_expr 获得输入类型
    match rhs_expr.infer_type(type_env, expr_env)? {
        // 因为此处产生的约束作用于外层环境, 而这些约束可能对再次推导 Apply 的类型有所帮助
        // 所以再次 infer_type 时应该将这些约束注入环境, 并对外传播
        rhs_expr_type @ (Triple::L(_) | Triple::M(_)) => {
            let (input_type, constraint) =
                rhs_expr_type.unwrap_type_constraint();

            let apply_expr = if let Some(output_type) = expect_type {
                // 可以确定输出类型
                let closure_type = Type::ClosureType(
                    input_type.clone().boxed(),
                    output_type.clone().boxed()
                );

                Expr::Apply(
                    expect_type.clone(),
                    lhs_expr
                        .with_fallback_type(&closure_type)
                        .boxed(),
                    rhs_expr.clone().boxed()
                )
            } else {
                let partial_closure_type = Type::PartialClosureType(
                    input_type.clone().boxed()
                );

                Expr::Apply(
                    None,
                    lhs_expr
                        .with_fallback_type(&partial_closure_type)
                        .boxed(),
                    rhs_expr.clone().boxed()
                )
            };

            let new_expr_env =
                expr_env.extend_constraint_new(constraint.clone());

            apply_expr
                .infer_type(type_env, &new_expr_env)?
                .with_constraint_acc(constraint)
        }

        Triple::R(ri) => ri.into()
    }
}
