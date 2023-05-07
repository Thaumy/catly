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
                mulF (int2F 8) (piSum (int2F 1) (int2F 6))
        ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr =
        Expr::EnvRef(namely_type!("Fraction"), "pi".to_string());
    let evaluated = eval_expr(&type_env, expr_env.boxed(), &expr);

    let r = Expr::Int(namely_type!("Int"), 0);

    assert_eq!(evaluated, r.ok());
}
