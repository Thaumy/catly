use crate::parser::expr::Expr;
use crate::type_checker::env::type_env::TypeEnv;
use crate::type_checker::r#type::TypeConstraint;

// 将模式匹配意义上的常量表达式解构为表达式环境注入
pub fn destruct_const_to_expr_env_inject<'t>(
    type_env: &TypeEnv,
    expr: &Expr
) -> Vec<(String, TypeConstraint)> {
    // TODO: 可使用生命周期优化
    match expr {
        Expr::EnvRef(t, n) => {
            let t = t
                .clone()
                .map(|t| TypeConstraint::Constraint(t))
                .unwrap_or(TypeConstraint::Free);

            vec![(n.to_string(), t)]
        }
        Expr::Struct(_, vec) =>
            vec.iter()
                .map(|(_, mt, e)| {
                    let e = e
                        .clone()
                        .try_with_fallback_type(&mt);
                    destruct_const_to_expr_env_inject(type_env, &e)
                })
                .flatten()
                .collect(): Vec<_>,
        _ => vec![]
    }
}
