use std::collections::HashMap;

use crate::env::r#type::env_ref_src::EnvRefSrc;
use crate::env::r#type::type_constraint::TypeConstraint;
use crate::env::r#type::type_env::TypeEnv;
use crate::get_type::r#fn::destruct_namely_type;
use crate::infra::option::AnyExt;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::Type;

// 将模式匹配意义上的常量表达式解构为表达式环境注入
pub fn destruct_match_const_to_expr_env_inject<'t>(
    type_env: &TypeEnv,
    expr: &Expr
) -> Vec<(String, TypeConstraint, EnvRefSrc)> {
    // 由于后续的 case_expr_type 会和 target_expr_type 进行相容性测试, 所以这里不负责类型检查
    // 另外在此处实施类型检查是极其复杂的, 这意味着要实现 get_type 的大部分功能
    match expr {
        Expr::EnvRef(mt, n) => {
            let tc = mt
                .as_ref()
                .map(|t| t.clone().into())
                .unwrap_or(TypeConstraint::Free);

            vec![(n.to_string(), tc, EnvRefSrc::NoSrc)]
        }
        Expr::Struct(t, vec) => {
            // 由于这里不负责类型检查, 所以可以转为无序的哈希表以提升检索效率
            let prod_fields = t.as_ref().and_then(|t| {
                match destruct_namely_type(type_env, &t) {
                    Some(Type::ProdType(vec)) =>
                        HashMap::<String, Type>::from_iter(
                            vec.iter()
                                .map(|(k, v)| (k.clone(), v.clone()))
                        )
                        .some(),
                    _ => None
                }
            });

            vec.iter()
                .map(|(n, mt, e)| {
                    // 简单地从 ProdType 中查找类型作为提示, 因为这里不负责类型检查
                    let prod_hint = prod_fields
                        .as_ref()
                        .and_then(|fields| fields.get(n).cloned());

                    let e = e
                        .try_with_fallback_type(&prod_hint)
                        .try_with_fallback_type(&mt);

                    destruct_match_const_to_expr_env_inject(
                        type_env, &e
                    )
                })
                .flatten()
                .collect(): Vec<_>
        }
        _ => vec![]
    }
}
