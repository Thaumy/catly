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
        def expt1: Int -> Int -> Int =
            b -> n ->
                let iter =
                    b -> count -> product ->
                        if intEq count 0 then
                            product
                        else
                            iter b (sub count 1) (mul b product)
                in
                    iter b n 1
        # 2
        def expt2 =
            b -> n ->
                let iter =
                    b -> count -> product ->
                        if intEq count 0 then
                            product
                        else
                            iter b (sub count 1) (mul b product)
                in
                    iter b n 1
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
        .get_ref("expt1")
        .unwrap();

    assert_eq!(get_type(&type_env, &expr_env, &expr), target_type())
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("expt2")
        .unwrap();

    assert_eq!(get_type(&type_env, &expr_env, &expr), target_type())
}
