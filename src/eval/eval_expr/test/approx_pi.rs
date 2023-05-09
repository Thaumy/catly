use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::parse_to_env;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::eval_expr::test::get_std_code;

fn gen_env<'t>() -> (TypeEnv<'t>, ExprEnv) {
    let seq = get_std_code() +
        "
        def approxF = n -> d ->
            if and (gt n 100000) (gt d 100000) then
                let
                    g = gcd n d
                in
                    if eq g 1 then
                        { n = div n 10000, d = div d 10000 }: Fraction
                    else
                        { n = div n g, d = div d g }: Fraction
            else
                { n = n, d = d }: Fraction

        def approxMul = a -> b ->
            if and (gt a 100000) (gt b 100000) then
                mul (div a 10000) (div b 10000)
            else
                mul a b

        def mulF = a -> b ->
            match a with
            | ({ n = an, d = ad }: Fraction) ->
                match b with
                | ({ n = bn, d = bd }: Fraction) ->
                    approxF (approxMul an bn) (approxMul ad bd)

        def divF = a -> b ->
            match a with
            | ({ n = an, d = ad }: Fraction) ->
                match b with
                | ({ n = bn, d = bd }: Fraction) ->
                    approxF (approxMul an bd) (approxMul ad bn)

        def addF = a -> b ->
            match a with
            | ({ n = an, d = ad }: Fraction) ->
                match b with
                | ({ n = bn, d = bd }: Fraction) ->
                    approxF (add (add an bd) (add bn ad)) (approxMul ad bd)

        def gtF = a -> b ->
            match a with
            | ({ n = an, d = ad }: Fraction) ->
                match b with
                | ({ n = bn, d = bd }: Fraction) ->
                    gt (approxMul an bd) (approxMul bn ad)

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

        def pi1 = pi 30
        def r1 = { n = 1299896093410000, d = 400986194984375 }: Fraction # 3.241747745
        ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("pi1")
        .unwrap();
    let evaluated = eval_expr(&type_env, eval_env, &ref_expr);

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("r1")
        .unwrap();
    let r = eval_expr(&type_env, eval_env, &ref_expr);

    assert_eq!(evaluated, r);
}
