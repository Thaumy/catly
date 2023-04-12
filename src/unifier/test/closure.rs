use std::ops::Deref;

use crate::env::type_env::TypeEnv;
use crate::infra::option::AnyExt;
use crate::infra::r#box::Ext;
use crate::parser::r#type::Type;
use crate::unifier::closure::lift as lift_closure;
use crate::unifier::lift;
use crate::unifier::unify;
use crate::{btree_set, int_type};

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
            Type::ClosureType(
                Type::TypeEnvRef("A".to_string()).boxed(),
                Type::TypeEnvRef("B".to_string()).boxed()
            )
        ),
        (
            "G".to_string(),
            Type::ClosureType(
                Type::TypeEnvRef("A".to_string()).boxed(),
                Type::ClosureType(
                    Type::TypeEnvRef("B".to_string()).boxed(),
                    Type::TypeEnvRef("C".to_string()).boxed()
                )
                .boxed()
            )
        ),
        (
            "FG".to_string(),
            Type::SumType(btree_set![
                Type::TypeEnvRef("F".to_string()),
                Type::TypeEnvRef("G".to_string()),
            ])
        ),
        (
            "AB".to_string(),
            Type::SumType(btree_set![
                Type::TypeEnvRef("A".to_string()),
                Type::TypeEnvRef("B".to_string()),
            ])
        ),
    ];

    TypeEnv::new(vec)
}

#[test]
fn test_lift_part1() {
    let env = &env();
    let a = &Type::TypeEnvRef("A".to_string());
    let b = &Type::TypeEnvRef("B".to_string());
    let derive = &Type::ClosureType(
        Type::TypeEnvRef("A".to_string()).boxed(),
        Type::TypeEnvRef("B".to_string()).boxed()
    );
    assert!(lift_closure(env, a, b.deref(), derive,));

    let base =
        &Type::ClosureType(a.clone().boxed(), b.clone().boxed());
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part2() {
    let env = &env();
    let a = &Type::TypeEnvRef("A".to_string());
    let b = &Type::TypeEnvRef("B".to_string());
    let derive = &Type::TypeEnvRef("F".to_string());
    assert!(lift_closure(env, a, b.deref(), derive,));

    let base =
        &Type::ClosureType(a.clone().boxed(), b.clone().boxed());
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part3() {
    let env = &env();
    let a = &Type::TypeEnvRef("A".to_string());
    let b = &Type::TypeEnvRef("B".to_string());
    let derive = &Type::TypeEnvRef("G".to_string());
    assert!(!lift_closure(env, a, b.deref(), derive,));

    let base =
        &Type::ClosureType(a.clone().boxed(), b.clone().boxed());
    assert!(!lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), None);
}

#[test]
fn test_lift_part4() {
    let env = &env();
    let a = &Type::TypeEnvRef("A".to_string());
    let b = &Type::TypeEnvRef("B".to_string());
    let derive = &Type::TypeEnvRef("FG".to_string());
    assert!(lift_closure(env, a, b.deref(), derive,));

    let base =
        &Type::ClosureType(a.clone().boxed(), b.clone().boxed());
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part5() {
    let env = &env();
    let a = &Type::TypeEnvRef("A".to_string());
    let b = &Type::TypeEnvRef("B".to_string());
    let derive = &Type::TypeEnvRef("AB".to_string());
    assert!(!lift_closure(env, a, b.deref(), derive,));

    let base =
        &Type::ClosureType(a.clone().boxed(), b.clone().boxed());
    assert!(!lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), None);
}
