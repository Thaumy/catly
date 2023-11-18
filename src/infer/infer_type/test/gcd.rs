use std::rc::Rc;

use crate::eval::std::std_code;
use crate::infer::env::closure_type;
use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::int_type;
use crate::infer::env::parse_to_env;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::test::check_has_type;

fn gen_env<'t>() -> (TypeEnv<'t>, Rc<ExprEnv>) {
    let seq = std_code().to_owned() +
        "
        # 1
        def gcd1: Int -> Int -> Int =
            a -> b ->
                if eq b 0 then
                    a
                else
                    gcd1 b (rem a b)
        # 2
        def gcd2 =
            a -> b ->
                if eq b 0 then
                    a
                else
                    gcd2 b (rem a b)
        # 3
        def gcd3 =
            a -> b ->
                match eq b 0 with
                | (_: True) -> a
                | _ -> gcd3 b (rem a b)
        ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("gcd1")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(
        int_type!(),
        closure_type!(int_type!(), int_type!())
    );
    check_has_type!(infer_result, t)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("gcd2")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(
        int_type!(),
        closure_type!(int_type!(), int_type!())
    );
    check_has_type!(infer_result, t)
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("gcd3")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(
        int_type!(),
        closure_type!(int_type!(), int_type!())
    );
    check_has_type!(infer_result, t)
}
