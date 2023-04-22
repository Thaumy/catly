use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::infer_type::r#fn::has_type;
use crate::infer_type::r#type::GetTypeReturn;
use crate::infer_type::test::integration::get_std_code;
use crate::infer_type::test::parse_env;
use crate::{closure_type, int_type};

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
    has_type(closure_type!(
        int_type!(),
        closure_type!(int_type!(), int_type!())
    ))
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("gcd1")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_eq!(expr_type, target_type())
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("gcd2")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_eq!(expr_type, target_type())
}
