use std::assert_matches::assert_matches;

use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::get_type::get_type;
use crate::get_type::r#type::TypeMissMatch;
use crate::get_type::test::parse_env;
use crate::infra::quad::Quad;
use crate::{
    bool_type,
    closure_type,
    has_type,
    int_type,
    namely_type,
    require_constraint,
    single_constraint,
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

        def apply7: Int = (_: Int -> Int) 1
        def apply8: Int = _ 1

        def apply9: Int = (a -> _) 1
        def apply10: Int = (a -> b -> c -> d -> 0) 1 2 3 4
        def apply11 = (a -> b -> c -> d -> 0) 1 2 3 4

        def a12 = a -> _
        def apply12: Int = a12 1
        def a13 = a -> b -> c -> d -> 0
        def apply13 = a13 1 2 3 4

        def apply14: Int -> Int = apply14

        def add15: Int -> Int -> Int = _
        def sub15: Int -> Int -> Int = _
        def apply15 = # fib
        n ->
            match n with
            | 0 -> 0
            | 1 -> 1
            | _ -> add15 (apply15 (sub15 n 1)) (apply15 (sub15 n 2))

        type True = Int
        type False = Int
        type Bool = True | False

        def true = 1: True
        def false = 0: False

        def eq: Int -> Int -> Bool = _

        type EmptyList = Unit
        type IntCons = { head: Int, tail: IntList }
        type IntList = IntCons | EmptyList

        def emptyList = (): EmptyList
        def intCons = h -> t -> { head = h, tail = t } : IntList

        def map: (Int -> Int) -> IntList -> IntList = # 16
            f -> list ->
                match list with
                | ({ head = head, tail = tail }: IntCons) ->
                    intCons (f head) (map f tail)
                | (_: EmptyList) -> emptyList

        def find: Int -> IntList -> Bool = # 17
            n -> list ->
                match list with
                | ({ head = head, tail = tail }: IntCons) ->
                    if eq head n then
                        true
                    else
                        find n tail
                | (_: EmptyList) -> false

        def filter: (Int -> Bool) -> IntList -> IntList = # 18
            p -> list ->
                match list with
                | ({ head = head, tail = tail }: IntCons) ->
                    if p head then
                        intCons head (filter p tail)
                        # { head = head, tail = filter p tail }
                    else
                        filter p tail
                | (_: EmptyList) -> emptyList
    ";
    parse_env(seq)
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply1")
        .unwrap();

    let r = has_type!(int_type!());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply2")
        .unwrap();

    let r = has_type!(unit_type!());

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply3")
        .unwrap();

    let r = has_type!(closure_type!(int_type!(), unit_type!()));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply4")
        .unwrap();

    assert_matches!(
        get_type(&type_env, &expr_env, &expr),
        Quad::R(TypeMissMatch { .. })
    )
}

#[test]
fn test_part5() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply5")
        .unwrap();

    assert_matches!(
        get_type(&type_env, &expr_env, &expr),
        Quad::R(TypeMissMatch { .. })
    )
}

#[test]
fn test_part6() {
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
fn test_part7() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply7")
        .unwrap();

    let r = has_type!(namely_type!("Int"));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part8() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply8")
        .unwrap();

    let r = has_type!(namely_type!("Int"));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part9() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply9")
        .unwrap();

    let r = has_type!(namely_type!("Int"));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part10() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply10")
        .unwrap();

    let r = has_type!(namely_type!("Int"));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part11() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply11")
        .unwrap();

    let r = has_type!(namely_type!("Int"));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part12() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply12")
        .unwrap();

    let r = require_constraint!(
        namely_type!("Int"),
        single_constraint!(
            "a12".to_string(),
            closure_type!(int_type!(), int_type!())
        )
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part13() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply13")
        .unwrap();

    let r = require_constraint!(
        namely_type!("Int"),
        single_constraint!(
            "a13".to_string(),
            closure_type!(
                int_type!(),
                closure_type!(
                    int_type!(),
                    closure_type!(
                        int_type!(),
                        closure_type!(int_type!(), int_type!())
                    )
                )
            )
        )
    );

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part14() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply14")
        .unwrap();

    let r = has_type!(closure_type!(int_type!(), int_type!()));

    assert_eq!(get_type(&type_env, &expr_env, &expr), r)
}

#[test]
fn test_part15() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("apply15")
        .unwrap();

    assert_eq!(
        get_type(&type_env, &expr_env, &expr),
        has_type!(closure_type!(int_type!(), int_type!()))
    )
}

#[test]
fn test_part16() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("map")
        .unwrap();

    assert_eq!(
        get_type(&type_env, &expr_env, &expr),
        has_type!(closure_type!(
            closure_type!(int_type!(), int_type!()),
            closure_type!(
                Type::NamelyType("IntList".to_string()),
                Type::NamelyType("IntList".to_string())
            )
        ))
    )
}

#[test]
fn test_part17() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("find")
        .unwrap();

    assert_eq!(
        get_type(&type_env, &expr_env, &expr),
        has_type!(closure_type!(
            int_type!(),
            closure_type!(
                Type::NamelyType("IntList".to_string()),
                bool_type!()
            )
        ))
    )
}

#[test]
fn test_part18() {
    let (type_env, expr_env) = gen_env();

    let expr = expr_env
        .get_ref("filter")
        .unwrap();

    assert_eq!(
        get_type(&type_env, &expr_env, &expr),
        has_type!(closure_type!(
            closure_type!(int_type!(), bool_type!()),
            closure_type!(
                Type::NamelyType("IntList".to_string()),
                Type::NamelyType("IntList".to_string())
            )
        ))
    )
}
