use std::rc::Rc;

use crate::eval::env::parse_to_env;
use crate::eval::env::ExprEnv;
use crate::eval::env::TypeEnv;
use crate::eval::eval_expr;
use crate::eval::std::std_code;
use crate::infra::WrapRc;

fn gen_env<'t>() -> (TypeEnv<'t>, Rc<ExprEnv>) {
    let seq = std_code().to_owned() +
        "
        # 1
        def fib1: Int -> Int =
            n ->
                match n with
                | 0 -> 0
                | 1 -> 1
                | _ -> add (fib1 (sub n 1)) (fib1 (sub n 2))
        # 2
        def fib2 =
            n ->
                match n with
                | 0 -> 0
                | 1 -> 1
                | _ -> add (fib2 (sub n 1)) (fib2 (sub n 2))

        # 3
        def fib3: Int -> Int =
            n ->
                let rec iter =
                    a -> b -> count ->
                        if eq count 0 then
                            b
                        else
                            iter (add a b) a (sub count 1)
                in
                    iter 1 0 n
        # 4
        def fib4 =
            n ->
                let rec iter =
                    a -> b -> count ->
                        if eq count 0 then
                            b
                        else
                            iter (add a b) a (sub count 1)
                in
                    iter 1 0 n

        def evalFib1 = fib1 4
        def r1 = 3
        def evalFib2 = fib2 5
        def r2 = 5
        def evalFib3 = fib3 6
        def r3 = 8
        def evalFib4 = fib4 7
        def r4 = 13
        ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("evalFib1")
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
        .get_ref_expr_and_env("evalFib2")
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
        .get_ref_expr_and_env("evalFib3")
        .unwrap();
    let evaluated =
        eval_expr(&type_env, &eval_env, &ref_expr.wrap_rc());

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("r3")
        .unwrap();
    let r = eval_expr(&type_env, &eval_env, &ref_expr.wrap_rc());

    assert_eq!(evaluated, r);
}

#[test]
fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("evalFib4")
        .unwrap();
    let evaluated =
        eval_expr(&type_env, &eval_env, &ref_expr.wrap_rc());

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("r4")
        .unwrap();
    let r = eval_expr(&type_env, &eval_env, &ref_expr.wrap_rc());

    assert_eq!(evaluated, r);
}
