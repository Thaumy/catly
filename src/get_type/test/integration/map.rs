use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::get_type::get_type;
use crate::get_type::r#type::GetTypeReturn;
use crate::get_type::test::integration::get_std_code;
use crate::get_type::test::parse_env;
use crate::{closure_type, has_type, int_type};

fn gen_env<'t>() -> (TypeEnv, ExprEnv<'t>) {
    let seq = get_std_code() +
        "
        # 1
        def map1: (Int -> Int) -> IntList -> IntList =
            f -> list ->
                match list with
                | ({ head = head, tail = tail }: IntCons) ->
                    intCons (f head) (map1 f tail)
                | (_: EmptyList) -> emptyList
        # 2
        def map2 =
            f -> list ->
                match list with
                | ({ head = head, tail = tail }: IntCons) ->
                    intCons (f head) (map2 f tail)
                | (_: EmptyList) -> emptyList
        ";
    parse_env(&seq)
}

fn target_type() -> GetTypeReturn {
    has_type!(closure_type!(
        closure_type!(int_type!(), int_type!()),
        closure_type!(
            Type::NamelyType("IntList".to_string()),
            Type::NamelyType("IntList".to_string())
        )
    ))
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("map1")
        .unwrap();

    assert_eq!(get_type(&type_env, &expr_env, &expr), target_type())
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("map2")
        .unwrap();

    assert_eq!(get_type(&type_env, &expr_env, &expr), target_type())
}
