use std::rc::Rc;

use crate::eval::env::parse_to_env;
use crate::eval::env::ExprEnv;
use crate::eval::env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::std::std_code;
use crate::infra::WrapRc;

fn gen_env<'t>() -> (TypeEnv<'t>, Rc<ExprEnv>) {
    let seq = std_code().to_owned().to_owned() +
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

        def evalExpt1 = expt1 2 4
        def r1 = 16
        def evalExpt2 = expt2 3 5
        def r2 = 243
        def evalExpt3 = expt3 5 7
        def r3 = 78125
        ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("evalExpt1")
        .unwrap();
    let evaluated =
        eval_expr(&type_env, &eval_env, &ref_expr.wrap_rc());

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("r1")
        .unwrap();
    let r = eval_expr(&type_env, &eval_env, &ref_expr.wrap_rc());

    assert_eq!(evaluated, r);
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("evalExpt2")
        .unwrap();
    let evaluated =
        eval_expr(&type_env, &eval_env, &ref_expr.wrap_rc());

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("r2")
        .unwrap();
    let r = eval_expr(&type_env, &eval_env, &ref_expr.wrap_rc());

    assert_eq!(evaluated, r);
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("evalExpt3")
        .unwrap();
    let evaluated =
        eval_expr(&type_env, &eval_env, &ref_expr.wrap_rc());

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("r3")
        .unwrap();
    let r = eval_expr(&type_env, &eval_env, &ref_expr.wrap_rc());

    assert_eq!(evaluated, r);
}
