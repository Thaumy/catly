use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::parse_env;
use crate::infer::env::r#macro::closure_type;
use crate::infer::env::r#macro::int_type;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::test::get_std_code;
use crate::infra::quad::Quad;

fn gen_env<'t>() -> (TypeEnv<'t>, ExprEnv<'t>) {
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
        ";
    parse_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("gcd1")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    match expr_type {
        Quad::L((t, e)) => {
            let r = closure_type!(
                int_type!(),
                closure_type!(int_type!(), int_type!())
            );
            assert_eq!(t, r);
            assert!(e.is_fully_typed());
        }
        _ => panic!()
    }
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("gcd2")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    match expr_type {
        Quad::L((t, e)) => {
            let r = closure_type!(
                int_type!(),
                closure_type!(int_type!(), int_type!())
            );
            assert_eq!(t, r);
            assert!(e.is_fully_typed());
        }
        _ => panic!()
    }
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("gcd3")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    match expr_type {
        Quad::L((t, e)) => {
            let r = closure_type!(
                int_type!(),
                closure_type!(int_type!(), int_type!())
            );
            assert_eq!(t, r);
            assert!(e.is_fully_typed());
        }
        _ => panic!()
    }
}
