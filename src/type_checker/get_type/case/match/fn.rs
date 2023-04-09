use crate::parser::expr::Expr;
use crate::type_checker::get_type::r#type::{
    ExprEnv,
    TypeConstraint,
    TypeEnv
};

// 将常量解构为表达式环境
pub fn destruct_const_to_expr_env(
    type_env: &TypeEnv,
    expr: &Expr
) -> ExprEnv {
    // TODO: 可使用生命周期优化
    match expr {
        Expr::EnvRef(t, n) => {
            let t = t
                .clone()
                .map(|t| TypeConstraint::Constraint(t))
                .unwrap_or_else(|| TypeConstraint::Free);

            vec![(n.to_string(), t)]
        }
        Expr::Struct(_, vec) =>
            vec.iter()
                .map(|(_, mt, e)| {
                    let e = e
                        .clone()
                        .try_with_fallback_type(&mt);
                    destruct_const_to_expr_env(type_env, &e)
                })
                .flatten()
                .collect(): Vec<_>,
        _ => vec![]
    }
}
