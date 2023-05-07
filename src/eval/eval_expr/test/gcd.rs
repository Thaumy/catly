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

        def evalGcd1 = gcd1 48 18
        def evalGcd2 = gcd2 42 56
        def evalGcd3 = gcd3 319 377
        ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("evalGcd1")
        .unwrap();
    let evaluated = eval_expr(&type_env, eval_env, &ref_expr);

    let r = Expr::Int(namely_type!("Int"), 6);

    assert_eq!(evaluated, r.ok());
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("evalGcd2")
        .unwrap();
    let evaluated = eval_expr(&type_env, eval_env, &ref_expr);

    let r = Expr::Int(namely_type!("Int"), 14);

    assert_eq!(evaluated, r.ok());
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("evalGcd3")
        .unwrap();
    let evaluated = eval_expr(&type_env, eval_env, &ref_expr);

    let r = Expr::Int(namely_type!("Int"), 29);

    assert_eq!(evaluated, r.ok());
}
