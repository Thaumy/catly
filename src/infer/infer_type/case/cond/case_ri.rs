use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::require_constraint::require_constraint;
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
) -> InferTypeRet {
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
        Quad::L(t) => require_constraint(t, constraint_acc),
        Quad::ML(rc) => rc.with_constraint_acc(constraint_acc),
        Quad::MR(ri) => ri.with_constraint_acc(constraint_acc),
        r => r
    }
}
