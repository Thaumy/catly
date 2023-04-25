use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::from_defines;
use crate::infer::env::type_env::TypeEnv;
use crate::parser::ast::parse_ast;
use crate::pp::preprocess;

mod apply;
mod closure;
mod cond;
mod discard;
mod env_ref;
mod int;
mod integration;
mod r#let;
mod r#match;
mod r#struct;
mod unit;

fn parse_env<'t>(seq: &str) -> (TypeEnv, ExprEnv<'t>) {
    let seq = preprocess(&seq).unwrap();
    let defines = parse_ast(seq).unwrap();

    from_defines(defines)
}
