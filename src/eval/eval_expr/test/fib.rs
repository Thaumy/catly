use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::parse_to_env;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::eval_expr::test::get_std_code;
use crate::eval::r#macro::namely_type;
use crate::eval::r#type::expr::Expr;
use crate::infra::r#box::BoxAnyExt;
use crate::infra::result::ResultAnyExt;

fn gen_env<'t>() -> (TypeEnv<'t>, ExprEnv) {
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

        def evalFib1 = fib1 4
        def evalFib2 = fib2 5
        def evalFib3 = fib3 6
        def evalFib4 = fib4 7
        ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("evalFib1")
        .unwrap();
    let evaluated = eval_expr(&type_env, eval_env, &ref_expr);

    let r = Expr::Int(namely_type!("Int"), 3);

    assert_eq!(evaluated, r.ok());
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("evalFib2")
        .unwrap();
    let evaluated = eval_expr(&type_env, eval_env, &ref_expr);

    let r = Expr::Int(namely_type!("Int"), 5);

    assert_eq!(evaluated, r.ok());
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("evalFib3")
        .unwrap();
    let evaluated = eval_expr(&type_env, eval_env, &ref_expr);

    let r = Expr::Int(namely_type!("Int"), 8);

    assert_eq!(evaluated, r.ok());
}

#[test]
fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("evalFib4")
        .unwrap();
    let evaluated = eval_expr(&type_env, eval_env, &ref_expr);

    let r = Expr::Int(namely_type!("Int"), 13);

    assert_eq!(evaluated, r.ok());
}
