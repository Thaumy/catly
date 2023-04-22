use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::get_type::r#fn::require_constraint_or_type;
use crate::get_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::get_type::r#type::require_constraint::require_extended_constraint;
use crate::get_type::r#type::require_info::RequireInfo;
use crate::get_type::r#type::GetTypeReturn;
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
        let rhs_expr_type = rhs_expr.infer_type(type_env, expr_env);
        match rhs_expr_type {
            // 因为此处产生的约束作用于外层环境, 而这些约束可能对再次推导 Apply 的类型有所帮助
            // 所以再次 get_type 时应该将这些约束注入环境, 并对外传播
            Quad::L(_) | Quad::ML(_) => {
                let (input_type, constraint_acc) = match rhs_expr_type
                {
                    Quad::L(input_type) =>
                        (input_type, EnvRefConstraint::empty()),
                    Quad::ML(rc) => (rc.r#type, rc.constraint),
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

                let new_expr_env = expr_env
                    .extend_constraint_new(constraint_acc.clone());

                // TODO: 考虑约束顺序对环境的影响
                match apply_expr.infer_type(type_env, &new_expr_env) {
                    Quad::L(t) =>
                        require_constraint_or_type(constraint_acc, t),
                    Quad::ML(rc) => require_extended_constraint(
                        rc.r#type,
                        constraint_acc,
                        rc.constraint.clone()
                    ),
                    mr_r => mr_r
                }
            }
            // 信息不足以获得 rhs_expr_type, 或类型不相容
            _ => Quad::MR(require_info)
        }
    } else {
        // 尝试从 rhs_expr 获得输入类型
        let rhs_expr_type = rhs_expr.infer_type(type_env, expr_env);
        match rhs_expr_type {
            // 注入约束并对外传播, 与上同理
            Quad::L(_) | Quad::ML(_) => {
                let (input_type, constraint_acc) = match rhs_expr_type
                {
                    Quad::L(input_type) =>
                        (input_type, EnvRefConstraint::empty()),
                    Quad::ML(rc) => (rc.r#type, rc.constraint),
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

                let new_expr_env = expr_env
                    .extend_constraint_new(constraint_acc.clone());

                match apply_expr.infer_type(type_env, &new_expr_env) {
                    Quad::L(t) =>
                        require_constraint_or_type(constraint_acc, t),
                    Quad::ML(rc) => require_extended_constraint(
                        rc.r#type,
                        constraint_acc,
                        rc.constraint.clone()
                    ),
                    mr_r => mr_r
                }
            }
            // 信息不足以获得 rhs_expr_type, 或类型不相容
            mr_r => mr_r
        }
    }
}
