use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::r#macro::namely_type;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#fn::has_type;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::test::integration::get_std_code;
use crate::infer::infer_type::test::parse_env;

fn gen_env<'t>() -> (TypeEnv<'t>, ExprEnv<'t>) {
    let seq = get_std_code() +
        "
        def gt: Int -> Int -> Bool = _

        type Fraction = { n: Int, d: Int }

        def fraction = n -> d ->
            { n = n, d = d }: Fraction

        def intToFraction = i ->
            fraction i 1

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
                    fraction (add an bn) (add ad bd)

        def gtF = a -> b ->
            match a with
            | ({ n = an, d = ad }: Fraction) ->
                match b with
                | ({ n = bn, d = bd}: Fraction) ->
                    gt (mul an bd) (mul bn ad)

        def pi =
            let piSum = a -> b ->
                if gtF a b then
                    intToFraction 0
                else
                    addF
                        (divF
                            (intToFraction 1)
                            (mulF a (addF a (intToFraction 2)))
                        )
                        (piSum
                            (addF a (intToFraction 4))
                            b
                        )
            in
                mulF (intToFraction 8) (piSum (intToFraction 1) (intToFraction 1000))
        ";
    parse_env(&seq)
}

fn target_type() -> InferTypeRet {
    has_type(namely_type!("Fraction"))
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("pi")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_eq!(expr_type, target_type())
}
