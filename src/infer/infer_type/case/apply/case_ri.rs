use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::require_constraint::require_constraint;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::quad::Quad;
use crate::infra::r#box::BoxAnyExt;
use crate::infra::triple::Triple;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::OptType;
use crate::parser::r#type::r#type::Type;

pub fn case_ri(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    constraint_acc: EnvRefConstraint,
    expect_type: &OptType,
    lhs_expr: &Expr,
    rhs_expr: &Expr
) -> InferTypeRet {
    // 可以确定输出类型
    if let Some(output_type) = expect_type {
        // 尝试从 rhs_expr 获得输入类型
        let rhs_expr_type = rhs_expr.infer_type(type_env, expr_env);
        match rhs_expr_type {
            // 因为此处产生的约束作用于外层环境, 而这些约束可能对再次推导 Apply 的类型有所帮助
            // 所以再次 infer_type 时应该将这些约束注入环境, 并对外传播
            Quad::L(_) | Quad::ML(_) => {
                let (input_type, constraint) =
                    rhs_expr_type.unwrap_type_and_constraint();
                let constraint_acc = match constraint_acc
                    .extend_new(constraint.clone())
                {
                    Some(c) => c,
                    None =>
                        return TypeMissMatch::of_constraint(
                            &constraint_acc,
                            &constraint
                        )
                        .into(),
                };

                let closure_type = Type::ClosureType(
                    input_type.clone().boxed(),
                    output_type.clone().boxed()
                );
                let apply_expr = Expr::Apply(
                    expect_type.clone(),
                    lhs_expr
                        .with_fallback_type(&closure_type)
                        .boxed(),
                    rhs_expr.clone().boxed()
                );

                let new_expr_env = expr_env
                    .extend_constraint_new(constraint_acc.clone());

                match apply_expr
                    .infer_type(type_env, &new_expr_env)?
                {
                    Triple::L(t) =>
                        require_constraint(t, constraint_acc),
                    Triple::M(rc) =>
                        rc.with_constraint_acc(constraint_acc),
                    Triple::R(ri) =>
                        ri.with_constraint_acc(constraint_acc),
                }
            }

            Quad::MR(ri) => ri.with_constraint_acc(constraint_acc),

            r => r
        }
    } else {
        // 尝试从 rhs_expr 获得输入类型
        let rhs_expr_type = rhs_expr.infer_type(type_env, expr_env);
        match rhs_expr_type {
            // 注入约束并对外传播, 与上同理
            Quad::L(_) | Quad::ML(_) => {
                let (input_type, constraint) =
                    rhs_expr_type.unwrap_type_and_constraint();
                let constraint_acc = match constraint_acc
                    .extend_new(constraint.clone())
                {
                    Some(c) => c,
                    None =>
                        return TypeMissMatch::of_constraint(
                            &constraint_acc,
                            &constraint
                        )
                        .into(),
                };

                let partial_closure_type = Type::PartialClosureType(
                    input_type.clone().boxed()
                );
                let apply_expr = Expr::Apply(
                    None,
                    lhs_expr
                        .with_fallback_type(&partial_closure_type)
                        .boxed(),
                    rhs_expr.clone().boxed()
                );

                let new_expr_env = expr_env
                    .extend_constraint_new(constraint_acc.clone());

                match apply_expr
                    .infer_type(type_env, &new_expr_env)?
                {
                    Triple::L(t) =>
                        require_constraint(t, constraint_acc),
                    Triple::M(rc) =>
                        rc.with_constraint_acc(constraint_acc),
                    Triple::R(ri) =>
                        ri.with_constraint_acc(constraint_acc),
                }
            }

            Quad::MR(ri) => ri.with_constraint_acc(constraint_acc),

            // 信息不足以获得 rhs_expr_type, 或类型不相容
            r => r
        }
    }
}
