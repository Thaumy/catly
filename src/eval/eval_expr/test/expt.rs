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

        def eval_expt1 = expt1 2 4
        def eval_expt2 = expt2 3 5
        def eval_expt3 = expt3 5 7
        ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("eval_expt1")
        .unwrap();
    let evaluated = eval_expr(&type_env, eval_env, &ref_expr);

    let r = Expr::Int(namely_type!("Int"), 16);

    assert_eq!(evaluated, r.ok());
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("eval_expt2")
        .unwrap();
    let evaluated = eval_expr(&type_env, eval_env, &ref_expr);

    let r = Expr::Int(namely_type!("Int"), 243);

    assert_eq!(evaluated, r.ok());
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("eval_expt3")
        .unwrap();
    let evaluated = eval_expr(&type_env, eval_env, &ref_expr);

    let r = Expr::Int(namely_type!("Int"), 78125);

    assert_eq!(evaluated, r.ok());
}
