use std::ops::Deref;

use crate::infer::env::type_env::TypeEnv;
use crate::infra::option::AnyExt;
use crate::infra::r#box::Ext;
use crate::parser::r#type::r#type::Type;
use crate::unify::closure::lift_closure;
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
    assert!(lift_closure(env, a, b.deref(), derive).is_some());

    let base = &closure_type!(a.clone(), b.clone());
    assert!(base
        .lift_to(env, derive)
        .is_some());
    assert_eq!(base.unify(env, derive), derive.clone().some());
}

#[test]
fn test_part2() {
    let env = &env();
    let a = &namely_type!("A");
    let b = &namely_type!("B");
    let derive = &namely_type!("F");
    assert!(lift_closure(env, a, b.deref(), derive).is_some());

    let base = &closure_type!(a.clone(), b.clone());
    assert!(base
        .lift_to(env, derive)
        .is_some());
    assert_eq!(base.unify(env, derive), derive.clone().some());
}

#[test]
fn test_part3() {
    let env = &env();
    let a = &namely_type!("A");
    let b = &namely_type!("B");
    let derive = &namely_type!("G");
    assert!(lift_closure(env, a, b.deref(), derive).is_none());

    let base = &closure_type!(a.clone(), b.clone());
    assert!(!base
        .lift_to(env, derive)
        .is_some());
    assert_eq!(base.unify(env, derive), None);
}

#[test]
fn test_part4() {
    let env = &env();
    let a = &namely_type!("A");
    let b = &namely_type!("B");
    let derive = &namely_type!("FG");
    assert!(lift_closure(env, a, b.deref(), derive).is_none());

    let base = &closure_type!(a.clone(), b.clone());
    assert!(base
        .lift_to(env, derive)
        .is_none());
    assert_eq!(base.unify(env, derive), None);
}

#[test]
fn test_part5() {
    let env = &env();
    let a = &namely_type!("A");
    let b = &namely_type!("B");
    let derive = &namely_type!("AB");
    assert!(lift_closure(env, a, b.deref(), derive).is_none());

    let base = &closure_type!(a.clone(), b.clone());
    assert!(!base
        .lift_to(env, derive)
        .is_some());
    assert_eq!(base.unify(env, derive), None);
}

#[test]
fn test_part6() {
    let env = &env();
    let a = &namely_type!("A");
    let b = &namely_type!("B");
    let derive = &Type::PartialClosureType(a.clone().boxed());
    let r = &Type::ClosureType(a.clone().boxed(), b.clone().boxed());
    assert_eq!(
        lift_closure(env, a, b.deref(), derive),
        r.clone().some()
    );

    let base = &closure_type!(a.clone(), b.clone());
    assert_eq!(base.lift_to(env, derive), r.clone().some());
    assert_eq!(base.unify(env, derive), r.clone().some());
}

#[test]
fn test_part7() {
    let env = &env();
    let a = &namely_type!("A");
    let b = &namely_type!("B");
    let base = &Type::PartialClosureType(a.clone().boxed());
    let derive =
        &Type::ClosureType(a.clone().boxed(), b.clone().boxed());

    assert_eq!(base.lift_to(env, derive), None);
    assert_eq!(base.unify(env, derive), derive.clone().some());
}
