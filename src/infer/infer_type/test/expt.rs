use std::rc::Rc;

use crate::eval::std::std_code;
use crate::infer::env::closure_type;
use crate::infer::env::int_type;
use crate::infer::env::parse_to_env;
use crate::infer::env::ExprEnv;
use crate::infer::env::TypeEnv;
use crate::infer::infer_type::test::check_has_type;

fn gen_env<'t>() -> (TypeEnv<'t>, Rc<ExprEnv>) {
    let seq = std_code().to_owned() +
        "
        # 1
        def expt1: Int -> Int -> Int =
            b -> n ->
                let rec iter =
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
                let rec iter =
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
                let rec iter =
                    b -> count -> product ->
                        match eq count 0 with
                        | (_: True) -> product
                        | (_: False) -> iter b (sub count 1) (mul b product)
                in
                    iter b n 1
        ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("expt1")
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
        .get_ref("expt2")
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
        .get_ref("expt3")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(
        int_type!(),
        closure_type!(int_type!(), int_type!())
    );
    check_has_type!(infer_result, t)
}
