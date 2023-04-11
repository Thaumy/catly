use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::infra::option::AnyExt;
use crate::parser::expr::Expr;
use crate::parser::r#type::Type;
use crate::type_checker::get_type::get_type;
use crate::type_checker::get_type::test::parse_env;
use crate::{has_type, require_info, type_miss_match};

fn gen_env<'t>() -> (TypeEnv, ExprEnv<'t>) {
    let seq = "
        type A = Int
    ";
    parse_env(seq)
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr =
        Expr::Discard(Type::TypeEnvRef("A".to_string()).some());

    assert_eq!(
        get_type(&type_env, &expr_env, &expr),
        has_type!(Type::TypeEnvRef("A".to_string()))
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

    let expr =
        Expr::Discard(Type::TypeEnvRef("B".to_string()).some());

    assert_eq!(
        get_type(&type_env, &expr_env, &expr),
        type_miss_match!()
    )
}
