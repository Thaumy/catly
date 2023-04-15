use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::infra::r#box::Ext;
use crate::type_checker::get_type::get_type;
use crate::type_checker::get_type::test::parse_env;
use crate::{
    closure_type,
    has_type,
    int_type,
    namely_type,
    require_constraint,
    single_constraint,
    type_miss_match,
    unit_type
};

fn gen_env<'t>() -> (TypeEnv, ExprEnv<'t>) {
    let seq = "
        def f1 = i -> i: Int
        def apply1 = f1 1

        def f2: Int -> Int -> Unit = a -> b -> ()
        def apply2 = f2 1 2

        def f3 = (a: Int) -> (b: Int) -> ()
        def apply3 = f3 1

        def apply4 = 1 1

        type F5 = Int -> Int
        def f5: F5 = a -> 1
        def apply5 = f5 1

        def b6 = _
        def apply6 = ((a: Int) -> 1) b6

        def apply7: Int = (a -> _) 1
        def apply8: Int = (a -> b -> c -> 0) 1 2 3
    ";
    parse_env(seq)
}

#[test]
pub fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply1")
        .unwrap();

    let r = has_type!(int_type!());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply2")
        .unwrap();

    let r = has_type!(unit_type!());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply3")
        .unwrap();

    let r = has_type!(closure_type!(int_type!(), unit_type!()));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply4")
        .unwrap();

    let r = type_miss_match!();

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part5() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply5")
        .unwrap();

    let r = type_miss_match!();

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part6() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply6")
        .unwrap();

    let r = require_constraint!(
        namely_type!("Int"),
        single_constraint!("b6".to_string(), namely_type!("Int"))
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part7() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply7")
        .unwrap();

    let r = has_type!(namely_type!("Int"));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
pub fn test_part8() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply8")
        .unwrap();

    let r = has_type!(namely_type!("Int"));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}
