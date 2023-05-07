use std::ops::Deref;

use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::r#type::expr::primitive_op::PrimitiveOp;
use crate::eval::r#type::expr::Expr;
use crate::eval::r#type::r#type::Type;
use crate::infra::either::{Either, EitherAnyExt};

pub fn source_lhs_to_closure<'t>(
    type_env: &'t TypeEnv,
    expr_env: Box<ExprEnv>,
    expr: &Expr
) -> Either<
    (Option<String>, Type, Expr, Box<ExprEnv>),
    (PrimitiveOp, Box<ExprEnv>)
> {
    match expr {
        Expr::EnvRef(_, ref_name) => {
            // TODO:
            // 此处为逐层查找 env_ref
            // 可以设置穿透的访问链, 提高 env_ref 的检索效率
            let (src_expr, src_env) = expr_env
                .get_src_expr_and_env(ref_name.as_str())
                .unwrap_or_else(|| {
                    panic!(
                        "EnvRef {ref_name:?} not found in expr env"
                    )
                });

            source_lhs_to_closure(type_env, src_env, src_expr)
        }

        Expr::Closure(
            _,
            input_name,
            input_type,
            output_expr,
            eval_env
        ) => (
            input_name.clone(),
            input_type.clone(),
            *output_expr.clone(),
            eval_env
                .clone()
                .map(|x| x.clone())
                .unwrap_or(expr_env.clone())
        )
            .l(),

        Expr::PrimitiveOp(_, op, eval_env) => (
            op.deref().clone(),
            eval_env
                .clone()
                .map(|x| x.clone())
                .unwrap_or(expr_env.clone())
        )
            .r(),
        _ => panic!("Impossible expr: {expr:?}")
    }
}
