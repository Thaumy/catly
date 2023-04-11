#![feature(type_ascription)]
#![feature(let_chains)]
#![feature(if_let_guard)]
extern crate core;

mod env;
mod infra;
pub mod parser;
pub mod type_checker;
pub mod unifier;
