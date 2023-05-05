use std::collections::HashMap;

use crate::infer::env::expr_env::{ExprEnv, ExprEnvEntry};
use crate::infer::env::type_env::{TypeEnv, TypeEnvEntry};
use crate::infra::option::OptionAnyExt;
use crate::parser::ast::parse_ast;
use crate::parser::define::Define;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::{OptType, Type};
use crate::pp::preprocess;

pub mod expr_env;
pub mod r#macro;
pub mod r#type;
pub mod type_env;

pub fn def_vec_to_def_map(
    def_vec: Vec<Define>
) -> Option<(HashMap<String, Type>, HashMap<String, (OptType, Expr)>)>
{
    def_vec.into_iter().try_fold(
        (HashMap::new(), HashMap::new()),
        |(mut type_env_map, mut expr_env_map), define| match define {
            Define::TypeDef(n, t) =>
                match type_env_map.insert(n, t) {
                    None => (type_env_map, expr_env_map).some(),
                    Some(_) => None
                },
            Define::ExprDef(n, t, e) => {
                match expr_env_map.insert(n, (t, e)) {
                    None => (type_env_map, expr_env_map).some(),
                    Some(_) => None
                }
            }
        }
    )
}

pub fn def_map_to_env_vec(
    type_def_map: HashMap<String, Type>,
    expr_def_map: HashMap<String, (OptType, Expr)>
) -> (Vec<TypeEnvEntry>, Vec<ExprEnvEntry>) {
    let type_env_vec = type_def_map
        .into_iter()
        .collect();
    let expr_env_vec = expr_def_map
        .into_iter()
        .map(|(x, (y, z))| (x, y.into(), z.into()))
        .collect();

    (type_env_vec, expr_env_vec)
}

pub fn parse_to_env<'t>(
    seq: &str
) -> Option<(TypeEnv<'t>, ExprEnv<'t>)> {
    let seq = preprocess(&seq)?;
    let def_vec = parse_ast(seq)?;

    let (type_def_map, expr_def_map) = def_vec_to_def_map(def_vec)?;
    let (type_env_vec, expr_env_vec) =
        def_map_to_env_vec(type_def_map, expr_def_map);

    (TypeEnv::new(type_env_vec), ExprEnv::new(expr_env_vec)).some()
}
