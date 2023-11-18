use std::rc::Rc;

use crate::infer::env::ExprEnv;
use crate::infer::env::TypeEnv;
use crate::infer::infer_type::InferTypeRet;
use crate::infra::Triple;
use crate::infra::WrapRc;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::OptType;
use crate::parser::r#type::Type;

pub fn case_ri(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    expect_type: &OptType,
    lhs_expr: &Expr,
    rhs_expr: &Expr
) -> InferTypeRet {
    // 尝试从 rhs_expr 获得输入类型
    match rhs_expr.infer_type(type_env, expr_env)? {
        // 因为此处产生的约束作用于外层环境, 而这些约束可能对再次推导 Apply 的类型有所帮助
        // 所以再次 infer_type 时应该将这些约束注入环境, 并对外传播
        result @ (Triple::L(_) | Triple::M(_)) => {
            let (typed_rhs_expr, constraint) =
                result.unwrap_expr_constraint();

            let input_type = typed_rhs_expr.unwrap_type_annot();

            let apply_expr = if let Some(output_type) = expect_type {
                // 可以确定输出类型
                let closure_type = Type::ClosureType(
                    input_type.clone().wrap_rc(),
                    output_type.clone().wrap_rc()
                );

                Expr::Apply(
                    expect_type.clone(),
                    // 使用类型标注不完备的 lhs_expr 是没有问题的, 因为它将在下一轮推导时完备化
                    lhs_expr
                        .with_fallback_type(&closure_type)
                        .wrap_rc(),
                    // TODO: 使用类型完备化表达式参与推导是否能够成为一种提供类型信息的新方式?
                    typed_rhs_expr
                        .clone()
                        .wrap_rc()
                )
            } else {
                let partial_closure_type = Type::PartialClosureType(
                    input_type.clone().wrap_rc()
                );

                Expr::Apply(
                    None,
                    // 与上同理
                    lhs_expr
                        .with_fallback_type(&partial_closure_type)
                        .wrap_rc(),
                    typed_rhs_expr
                        .clone()
                        .wrap_rc()
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
