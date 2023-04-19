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
        def gcd1: Int -> Int -> Int =
            a -> b ->
                if intEq b 0 then
                    a
                else
                    gcd1 b (rem a b)
        # 2
        def gcd2 =
            a -> b ->
                if intEq b 0 then
                    a
                else
                    gcd2 b (rem a b)
        ";
    parse_env(&seq)
}

fn target_type() -> GetTypeReturn {
    has_type!(closure_type!(
        int_type!(),
        closure_type!(int_type!(), int_type!())
    ))
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("gcd1")
        .unwrap();

    assert_eq!(get_type(&type_env, &expr_env, &expr), target_type())
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("gcd2")
        .unwrap();

    assert_eq!(get_type(&type_env, &expr_env, &expr), target_type())
}
