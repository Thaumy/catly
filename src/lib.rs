#![feature(type_ascription)]
#![feature(let_chains)]
#![feature(if_let_guard)]
#![feature(assert_matches)]
#![feature(iterator_try_reduce)]
#![feature(iterator_try_collect)]
#![feature(try_trait_v2)]
#![feature(try_blocks)]
extern crate core;

pub mod eval;
pub mod infer;
pub mod infra;
pub mod lexer;
pub mod parser;
pub mod pp;
pub mod unify;
