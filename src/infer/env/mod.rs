use std::collections::HashMap;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infra::option::OptionAnyExt;
use crate::infra::vec::VecExt;
use crate::parser::ast::parse_ast;
use crate::parser::define::Define;
use crate::pp::preprocess;

pub mod expr_env;
pub mod r#macro;
pub mod r#type;
pub mod type_env;

fn from_define_vec<'t>(
    value: Vec<Define>
) -> Option<(TypeEnv<'t>, ExprEnv<'t>)> {
    let (type_env_map, expr_env_map) = value.into_iter().try_fold(
        (HashMap::new(), HashMap::new()),
        |(mut type_env_map, mut expr_env_map), define| match define {
            Define::TypeDef(n, t) =>
                match type_env_map.insert(n, t) {
                    None => (type_env_map, expr_env_map).some(),
                    Some(_) => None
                },
            Define::ExprDef(n, t, e) => {
                match expr_env_map.insert(n, (t.into(), e.into())) {
                    None => (type_env_map, expr_env_map).some(),
                    Some(_) => None
                }
            }
        }
    )?;

    let type_env_vec = type_env_map
        .into_iter()
        .collect();
    let expr_env_vec = expr_env_map
        .into_iter()
        .map(|(x, (y, z))| (x, y, z))
        .collect();

    (TypeEnv::new(type_env_vec), ExprEnv::new(expr_env_vec)).some()
}

pub fn parse_env<'t>(
    seq: &str
) -> Option<(TypeEnv<'t>, ExprEnv<'t>)> {
    let seq = preprocess(&seq)?;
    let defines = parse_ast(seq)?;

    from_define_vec(defines)
}
