use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::infer_type::r#fn::require_constraint_or_type;
use crate::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer_type::r#type::require_constraint::require_extended_constraint;
use crate::infer_type::r#type::GetTypeReturn;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::infra::r#box::Ext;
use crate::parser::expr::r#type::Expr;

pub fn case_ri(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    bool_expr: &Expr,
    else_expr: &Expr,
    then_expr: &Expr
) -> GetTypeReturn {
    let (else_expr_type, constraint_acc) =
        match else_expr.infer_type(type_env, expr_env) {
            Quad::L(t) => (t, EnvRefConstraint::empty()),
            // 需要收集这些作用于外层环境的约束并传播, 因为它们可能对推导 then_expr_type 有所帮助
            Quad::ML(rc) => (rc.r#type, rc.constraint),
            mr_r => return mr_r
        };

    let cond_expr = Expr::Cond(
        else_expr_type.some(),
        bool_expr.clone().boxed(),
        then_expr.clone().boxed(),
        else_expr.clone().boxed()
    );

    let new_expr_env =
        expr_env.extend_constraint_new(constraint_acc.clone());

    match cond_expr.infer_type(type_env, &new_expr_env) {
        Quad::L(t) => require_constraint_or_type(constraint_acc, t),
        Quad::ML(rc) => require_extended_constraint(
            rc.r#type,
            constraint_acc,
            rc.constraint.clone()
        ),
        mr_r => mr_r
    }
}
