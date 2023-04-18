use std::assert_matches::assert_matches;

use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::get_type::get_type;
use crate::get_type::r#type::TypeMissMatch;
use crate::get_type::test::parse_env;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::parser::expr::r#type::Expr;
use crate::{has_type, namely_type, require_info};

fn gen_env<'t>() -> (TypeEnv, ExprEnv<'t>) {
    let seq = "
        type A = Int
    ";
    parse_env(seq)
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr = Expr::Discard(namely_type!("A").some());

    assert_eq!(
        get_type(&type_env, &expr_env, &expr),
        has_type!(namely_type!("A"))
    )
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr = Expr::Discard(None);

    assert_eq!(
        get_type(&type_env, &expr_env, &expr),
        require_info!("_".to_string())
    )
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr = Expr::Discard(namely_type!("B").some());

    assert_matches!(
        get_type(&type_env, &expr_env, &expr),
        Quad::R(TypeMissMatch { .. })
    )
}
