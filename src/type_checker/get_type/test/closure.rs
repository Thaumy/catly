use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::infra::r#box::Ext;
use crate::parser::r#type::Type;
use crate::type_checker::get_type::get_type;
use crate::type_checker::get_type::r#type::EnvRefConstraint;
use crate::type_checker::get_type::test::parse_env;
use crate::{
    has_type,
    int_type,
    require_constraint,
    require_info,
    unit_type
};

fn gen_env<'t>() -> (TypeEnv, ExprEnv<'t>) {
    let seq = "
        def f1 = a -> a
        def f2: Int -> Int = a -> a
        def f3 = (a: Int) -> a
        def f4 = a -> a: Int

        def b = _
        def f5: Int -> Int = a -> b

        def f6: Int -> Int -> Unit = a -> a -> ()
        type F = Int -> Int -> Unit
        def f7: F = a -> a -> ()

        def f8: (Int -> Unit) -> Int = f -> 1

        def x = _
        def f9: Int -> Int -> Int = a -> x -> x
        def f10: Int -> (Int -> Int) = a -> (b -> x)
    ";
    parse_env(seq)
}

#[test]
pub fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f1")
        .unwrap();

    let r = require_info!("a".to_string());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f2")
        .unwrap();

    let r = has_type!(Type::ClosureType(
        int_type!().boxed(),
        int_type!().boxed(),
    ));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f3")
        .unwrap();

    let r = has_type!(Type::ClosureType(
        int_type!().boxed(),
        int_type!().boxed(),
    ));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f4")
        .unwrap();

    let r = has_type!(Type::ClosureType(
        int_type!().boxed(),
        int_type!().boxed(),
    ));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part5() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f5")
        .unwrap();
    let r = require_constraint!(
        Type::ClosureType(int_type!().boxed(), int_type!().boxed(),),
        EnvRefConstraint::single("b".to_string(), int_type!())
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part6() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f6")
        .unwrap();

    let r = has_type!(Type::ClosureType(
        int_type!().boxed(),
        Type::ClosureType(int_type!().boxed(), unit_type!().boxed())
            .boxed()
    ));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part7() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f7")
        .unwrap();

    let r = has_type!(Type::TypeEnvRef("F".to_string()));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part8() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f8")
        .unwrap();

    let r = has_type!(Type::ClosureType(
        Type::ClosureType(int_type!().boxed(), unit_type!().boxed())
            .boxed(),
        int_type!().boxed()
    ));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part9() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f9")
        .unwrap();

    let r = has_type!(Type::ClosureType(
        int_type!().boxed(),
        Type::ClosureType(int_type!().boxed(), int_type!().boxed())
            .boxed()
    ));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part10() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("f10")
        .unwrap();

    let r = require_constraint!(
        Type::ClosureType(
            int_type!().boxed(),
            Type::ClosureType(
                int_type!().boxed(),
                int_type!().boxed()
            )
            .boxed()
        ),
        EnvRefConstraint::single("x".to_string(), int_type!())
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}
