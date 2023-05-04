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
                let iter =
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
                let iter =
                    a -> b -> count ->
                        if eq count 0 then
                            b
                        else
                            iter (add a b) a (sub count 1)
                in
                    iter 1 0 n
        ";
    parse_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("fib1")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(int_type!(), int_type!());
    check_has_type!(infer_result, t)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("fib2")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(int_type!(), int_type!());
    check_has_type!(infer_result, t)
}
#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("fib3")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(int_type!(), int_type!());
    check_has_type!(infer_result, t)
}
#[test]
fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("fib4")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(int_type!(), int_type!());
    check_has_type!(infer_result, t)
}
