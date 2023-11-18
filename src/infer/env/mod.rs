use std::collections::HashMap;
use std::rc::Rc;

use crate::infra::WrapOption;
use crate::infra::WrapRc;
use crate::lexer::lexical_analyze;
use crate::parser::ast::parse_ast;
use crate::parser::define::Define;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::{OptType, Type};
use crate::pp::preprocess;

mod expr_env;
mod r#macro;
mod r#type;
mod type_env;

pub use expr_env::*;
pub use r#macro::*;
pub use r#type::*;
pub use type_env::*;

pub fn def_vec_to_def_map(
    def_vec: Vec<Define>
) -> Option<(HashMap<String, Type>, HashMap<String, (OptType, Expr)>)>
{
    def_vec.into_iter().try_fold(
        (HashMap::new(), HashMap::new()),
        |(mut type_env_map, mut expr_env_map), define| match define {
            Define::TypeDef(n, t) =>
                match type_env_map.insert(n, t) {
                    None => (type_env_map, expr_env_map).wrap_some(),
                    Some(_) => None
                },
            Define::ExprDef(n, t, e) => {
                match expr_env_map.insert(n, (t, e)) {
                    None => (type_env_map, expr_env_map).wrap_some(),
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
) -> Option<(TypeEnv<'t>, Rc<ExprEnv>)> {
    let preprocessed = preprocess(seq);
    let tokens = lexical_analyze(preprocessed.as_str())?;
    let def_vec = parse_ast(tokens)?;

    let (type_def_map, expr_def_map) = def_vec_to_def_map(def_vec)?;
    let (type_env_vec, expr_env_vec) =
        def_map_to_env_vec(type_def_map, expr_def_map);

    let type_env = TypeEnv::new(type_env_vec);
    let expr_env = ExprEnv::empty()
        .wrap_rc()
        .extend_vec_new(expr_env_vec);

    (type_env, expr_env).wrap_some()
}
