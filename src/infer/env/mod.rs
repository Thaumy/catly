use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infra::vec::Ext;
use crate::parser::ast::parse_ast;
use crate::parser::define::Define;
use crate::pp::preprocess;

pub mod expr_env;
pub mod r#macro;
pub mod r#type;
pub mod type_env;

fn from_defines<'t>(
    defines: Vec<Define>
) -> (TypeEnv<'t>, ExprEnv<'t>) {
    let (tev, eev) = defines.iter().fold(
        (vec![], vec![]),
        |(tev, eev), define| match define {
            Define::TypeDef(n, t) =>
                (tev.chain_push((n.clone(), t.clone())), eev),
            Define::ExprDef(n, et, ee) => {
                let tc = et.clone().into();
                (
                    tev,
                    eev.chain_push((
                        n.clone(),
                        tc,
                        ee.clone().into()
                    ))
                )
            }
        }
    );

    (TypeEnv::new(tev), ExprEnv::new(eev))
}

pub fn parse_env<'t>(seq: &str) -> (TypeEnv<'t>, ExprEnv<'t>) {
    let seq = preprocess(&seq).unwrap();
    let defines = parse_ast(seq).unwrap();

    from_defines(defines)
}
