use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::get_type::get_type;
use crate::get_type::r#type::{GetTypeReturn, RequireInfo};
use crate::infra::alias::MaybeType;
use crate::infra::quad::Quad;
use crate::infra::r#box::Ext;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::Type;

pub fn case_ri(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    require_info: RequireInfo,
    expect_type: &MaybeType,
    lhs_expr: &Expr,
    rhs_expr: &Expr
) -> GetTypeReturn {
    // 可以确定输出类型
    if let Some(output_type) = expect_type {
        // 尝试从 rhs_expr 获得输入类型
        let rhs_expr_type = get_type(type_env, expr_env, rhs_expr);
        match rhs_expr_type {
            // 约束将在调用 get_type 时被传播, 所以无需处理
            Quad::L(_) | Quad::ML(_) => {
                let input_type = match rhs_expr_type {
                    Quad::L(input_type) => input_type,
                    Quad::ML(rc) => rc.r#type,
                    _ => panic!(
                        "Impossible rhs_expr_type: {rhs_expr_type:?}"
                    )
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

                get_type(type_env, expr_env, &apply_expr)
            }
            // 信息不足以获得 rhs_expr_type, 或类型不相容
            _ => Quad::MR(require_info)
        }
    } else {
        // 尝试从 rhs_expr 获得输入类型
        let rhs_expr_type = get_type(type_env, expr_env, rhs_expr);
        match rhs_expr_type {
            // 约束将在调用 get_type 时被传播, 所以无需处理
            Quad::L(_) | Quad::ML(_) => {
                let input_type = match rhs_expr_type {
                    Quad::L(input_type) => input_type,
                    Quad::ML(rc) => rc.r#type,
                    _ => panic!(
                        "Impossible rhs_expr_type: {rhs_expr_type:?}"
                    )
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

                get_type(type_env, expr_env, &apply_expr)
            }
            // 信息不足以获得 rhs_expr_type, 或类型不相容
            mr_r => mr_r
        }
    }
}
