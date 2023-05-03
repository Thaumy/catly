use std::assert_matches::assert_matches;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::parse_env;
use crate::infer::env::r#macro::closure_type;
use crate::infer::env::r#macro::int_type;
use crate::infer::env::r#macro::namely_type;
use crate::infer::env::r#macro::unit_type;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infra::quad::Quad;

fn gen_env<'t>() -> (TypeEnv<'t>, ExprEnv<'t>) {
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
        def apply15: Unit = (a -> 1) 1
        def apply16 = ((a: Unit) -> 1) 1
        def a17 = _
        def apply17 = ((a: Unit) -> 1) ((let a = a17: Int in 1): Int)
        def apply18 = ((a: Int) -> 1) ((): Unit)
    ";
    parse_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("apply1")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    match expr_type {
        Quad::L((t, e)) => {
            let r = int_type!();
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
        .get_ref("apply2")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    match expr_type {
        Quad::L((t, e)) => {
            let r = unit_type!();
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
        .get_ref("apply3")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    match expr_type {
        Quad::L((t, e)) => {
            let r = closure_type!(int_type!(), unit_type!());
            assert_eq!(t, r);
            assert!(e.is_fully_typed());
        }
        _ => panic!()
    }
}

#[test]
fn test_part4() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("apply4")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(expr_type, Quad::R(..))
}

#[test]
fn test_part5() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("apply5")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(expr_type, Quad::R(..))
}

#[test]
fn test_part6() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("apply6")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    match expr_type {
        Quad::ML(rc) => {
            let r = namely_type!("Int");
            assert_eq!(rc.r#type, r);
            let r = EnvRefConstraint::single(
                "b6".to_string(),
                namely_type!("Int")
            );
            assert_eq!(rc.constraint, r);
            assert!(rc.typed_expr.is_fully_typed());
        }
        _ => panic!()
    }
}

#[test]
fn test_part7() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("apply7")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    match expr_type {
        Quad::L((t, e)) => {
            let r = namely_type!("Int");
            assert_eq!(t, r);
            assert!(e.is_fully_typed());
        }
        _ => panic!()
    }
}

#[test]
fn test_part8() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("apply8")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    match expr_type {
        Quad::L((t, e)) => {
            let r = namely_type!("Int");
            assert_eq!(t, r);
            assert!(e.is_fully_typed());
        }
        _ => panic!()
    }
}

#[test]
fn test_part9() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("apply9")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    match expr_type {
        Quad::L((t, e)) => {
            let r = namely_type!("Int");
            assert_eq!(t, r);
            assert!(e.is_fully_typed());
        }
        _ => panic!()
    }
}

#[test]
fn test_part10() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("apply10")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    match expr_type {
        Quad::L((t, e)) => {
            let r = namely_type!("Int");
            assert_eq!(t, r);
            assert!(e.is_fully_typed());
        }
        _ => panic!()
    }
}

#[test]
fn test_part11() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("apply11")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    match expr_type {
        Quad::L((t, e)) => {
            let r = namely_type!("Int");
            assert_eq!(t, r);
            assert!(e.is_fully_typed());
        }
        _ => panic!()
    }
}

#[test]
fn test_part12() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("apply12")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    match expr_type {
        Quad::ML(rc) => {
            let r = namely_type!("Int");
            assert_eq!(rc.r#type, r);
            let r = EnvRefConstraint::single(
                "a12".to_string(),
                closure_type!(int_type!(), int_type!())
            );
            assert_eq!(rc.constraint, r);
            assert!(rc.typed_expr.is_fully_typed());
        }
        _ => panic!()
    }
}

#[test]
fn test_part13() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("apply13")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    match expr_type {
        Quad::ML(rc) => {
            let r = namely_type!("Int");
            assert_eq!(rc.r#type, r);
            let r = EnvRefConstraint::single(
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
            );
            assert_eq!(rc.constraint, r);
            assert!(rc.typed_expr.is_fully_typed());
        }
        _ => panic!()
    }
}

#[test]
fn test_part14() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("apply14")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    match expr_type {
        Quad::L((t, e)) => {
            let r = closure_type!(int_type!(), int_type!());
            assert_eq!(t, r);
            assert!(e.is_fully_typed());
        }
        _ => panic!()
    }
}

#[test]
fn test_part15() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("apply15")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(expr_type, Quad::R { .. })
}

#[test]
fn test_part16() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("apply16")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(expr_type, Quad::R { .. })
}

#[test]
fn test_part17() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("apply17")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(expr_type, Quad::R { .. })
}

#[test]
fn test_part18() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("apply18")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(expr_type, Quad::R { .. })
}
