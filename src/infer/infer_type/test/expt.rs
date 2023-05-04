use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::parse_env;
use crate::infer::env::r#macro::closure_type;
use crate::infer::env::r#macro::int_type;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::test::{check_has_type, get_std_code};

fn gen_env<'t>() -> (TypeEnv<'t>, ExprEnv<'t>) {
    let seq = get_std_code() +
        "
        # 1
        def expt1: Int -> Int -> Int =
            b -> n ->
                let iter =
                    b -> count -> product ->
                        if eq count 0 then
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
                        if eq count 0 then
                            product
                        else
                            iter b (sub count 1) (mul b product)
                in
                    iter b n 1
        # 3
        def expt3 =
            b -> n ->
                let iter =
                    b -> count -> product ->
                        match eq count 0 with
                        | (_: True) -> product
                        | (_: False) -> iter b (sub count 1) (mul b product)
                in
                    iter b n 1
        ";
    parse_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("expt1")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(
        int_type!(),
        closure_type!(int_type!(), int_type!())
    );
    check_has_type!(expr_type, t)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("expt2")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(
        int_type!(),
        closure_type!(int_type!(), int_type!())
    );
    check_has_type!(expr_type, t)
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("expt3")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(
        int_type!(),
        closure_type!(int_type!(), int_type!())
    );
    check_has_type!(expr_type, t)
}
