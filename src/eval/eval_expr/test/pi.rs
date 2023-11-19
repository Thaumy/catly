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
        def mulF = a -> b ->
            match a with
            | ({ n = an, d = ad }: Fraction) ->
                match b with
                | ({ n = bn, d = bd }: Fraction) ->
                    fraction (mul an bn) (mul ad bd)

        def divF = a -> b ->
            match a with
            | ({ n = an, d = ad }: Fraction) ->
                match b with
                | ({ n = bn, d = bd }: Fraction) ->
                    fraction (mul an bd) (mul ad bn)

        def addF = a -> b ->
            match a with
            | ({ n = an, d = ad }: Fraction) ->
                match b with
                | ({ n = bn, d = bd }: Fraction) ->
                    fraction (add (add an bd) (add bn ad)) (mul ad bd)

        def gtF = a -> b ->
            match a with
            | ({ n = an, d = ad }: Fraction) ->
                match b with
                | ({ n = bn, d = bd }: Fraction) ->
                    gt (mul an bd) (mul bn ad)

        def pi = n ->
            let rec piSum = a -> b ->
                if gtF a b then
                    int2F 0
                else
                    addF
                        (divF
                            (int2F 1)
                            (mulF a (addF a (int2F 2)))
                        )
                        (piSum
                            (addF a (int2F 4))
                            b
                        )
            in
                mulF (int2F 16) (piSum (int2F 1) (int2F n))

        def pi1 = pi 10
        def r1 = fraction 2592 385 # 6.732467532
        def pi2 = pi 20
        def r2 = fraction 120547008 37182145 # 3.242067073
        def pi3 = pi 30
        def r3 = fraction 54617481250096 16848159453125 # 3.241747646
        ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("pi1")
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
        .get_ref_expr_and_env("pi2")
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
        .get_ref_expr_and_env("pi3")
        .unwrap();
    let evaluated =
        eval_expr(&type_env, &eval_env, &ref_expr.wrap_rc());

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("r3")
        .unwrap();
    let r = eval_expr(&type_env, &eval_env, &ref_expr.wrap_rc());

    assert_eq!(evaluated, r);
}
