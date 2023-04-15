use std::ops::Deref;

use crate::env::type_env::TypeEnv;
use crate::infra::option::AnyExt;
use crate::infra::r#box::Ext;
use crate::unifier::closure::lift as lift_closure;
use crate::unifier::lift;
use crate::unifier::unify;
use crate::{
    btree_set,
    closure_type,
    int_type,
    namely_type,
    sum_type
};

fn env() -> TypeEnv {
    /* env:
    type A = Int
    type B = Int

    type F = A -> B
    type G = A -> B -> C
    type FG = F | G
    type AB = A | B
    */
    let vec = vec![
        ("A".to_string(), int_type!()),
        ("B".to_string(), int_type!()),
        (
            "F".to_string(),
            closure_type!(namely_type!("A"), namely_type!("B"))
        ),
        (
            "G".to_string(),
            closure_type!(
                namely_type!("A"),
                closure_type!(namely_type!("B"), namely_type!("C"))
            )
        ),
        ("FG".to_string(), sum_type![
            namely_type!("F"),
            namely_type!("G"),
        ]),
        ("AB".to_string(), sum_type![
            namely_type!("A"),
            namely_type!("B"),
        ]),
    ];

    TypeEnv::new(vec)
}

#[test]
fn test_part1() {
    let env = &env();
    let a = &namely_type!("A");
    let b = &namely_type!("B");
    let derive = &closure_type!(namely_type!("A"), namely_type!("B"));
    assert!(lift_closure(env, a, b.deref(), derive,));

    let base = &closure_type!(a.clone(), b.clone());
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_part2() {
    let env = &env();
    let a = &namely_type!("A");
    let b = &namely_type!("B");
    let derive = &namely_type!("F");
    assert!(lift_closure(env, a, b.deref(), derive,));

    let base = &closure_type!(a.clone(), b.clone());
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_part3() {
    let env = &env();
    let a = &namely_type!("A");
    let b = &namely_type!("B");
    let derive = &namely_type!("G");
    assert!(!lift_closure(env, a, b.deref(), derive,));

    let base = &closure_type!(a.clone(), b.clone());
    assert!(!lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), None);
}

#[test]
fn test_part4() {
    let env = &env();
    let a = &namely_type!("A");
    let b = &namely_type!("B");
    let derive = &namely_type!("FG");
    assert!(lift_closure(env, a, b.deref(), derive,));

    let base = &closure_type!(a.clone(), b.clone());
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_part5() {
    let env = &env();
    let a = &namely_type!("A");
    let b = &namely_type!("B");
    let derive = &namely_type!("AB");
    assert!(!lift_closure(env, a, b.deref(), derive,));

    let base = &closure_type!(a.clone(), b.clone());
    assert!(!lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), None);
}
