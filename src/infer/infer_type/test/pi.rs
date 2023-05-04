use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::parse_env;
use crate::infer::env::r#macro::namely_type;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::test::{check_has_type, get_std_code};

fn gen_env<'t>() -> (TypeEnv<'t>, ExprEnv<'t>) {
    let seq = get_std_code() +
        "
        def mulF = a -> b ->
            match a with
            | ({ n = an, d = ad }: Fraction) ->
                match b with
                | ({ n = bn, d = bd}: Fraction) ->
                    fraction (mul an bn) (mul ad bd)

        def divF = a -> b ->
            match a with
            | ({ n = an, d = ad }: Fraction) ->
                match b with
                | ({ n = bn, d = bd}: Fraction) ->
                    fraction (mul an bd) (mul ad bn)

        def addF = a -> b ->
            match a with
            | ({ n = an, d = ad }: Fraction) ->
                match b with
                | ({ n = bn, d = bd}: Fraction) ->
                    fraction (add (add an bd) (add bn ad)) (mul ad bd)

        def gtF = a -> b ->
            match a with
            | ({ n = an, d = ad }: Fraction) ->
                match b with
                | ({ n = bn, d = bd}: Fraction) ->
                    gt (mul an bd) (mul bn ad)

        def pi =
            let piSum = a -> b ->
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
                mulF (int2F 8) (piSum (int2F 1) (int2F 1000))
        ";
    parse_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("pi")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = namely_type!("Fraction");
    check_has_type!(infer_result, t)
}
