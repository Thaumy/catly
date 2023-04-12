use crate::env::type_env::TypeEnv;
use crate::infra::option::AnyExt;
use crate::parser::r#type::Type;
use crate::unifier::env_ref::lift as lift_env_ref;
use crate::unifier::lift;
use crate::unifier::unify;
use crate::{btree_set, int_type};

fn env() -> TypeEnv {
    /* env:
    type A = Int
    type B = A
    type C = B

    type S0 = D | Int
    type S1 = D | A
    type S2 = D | C
    type S3 = D | E
    */
    let vec = vec![
        ("A".to_string(), int_type!()),
        ("B".to_string(), Type::TypeEnvRef("A".to_string())),
        ("C".to_string(), Type::TypeEnvRef("B".to_string())),
        (
            "S0".to_string(),
            Type::SumType(btree_set![
                Type::TypeEnvRef("D".to_string()),
                int_type!(),
            ])
        ),
        (
            "S1".to_string(),
            Type::SumType(btree_set![
                Type::TypeEnvRef("D".to_string()),
                Type::TypeEnvRef("A".to_string()),
            ])
        ),
        (
            "S2".to_string(),
            Type::SumType(btree_set![
                Type::TypeEnvRef("D".to_string()),
                Type::TypeEnvRef("C".to_string()),
            ])
        ),
        (
            "S3".to_string(),
            Type::SumType(btree_set![
                Type::TypeEnvRef("D".to_string()),
                Type::TypeEnvRef("E".to_string()),
            ])
        ),
    ];

    TypeEnv::new(vec)
}

#[test]
fn test_lift_part1() {
    let env = &env();
    let derive = &int_type!();
    assert!(lift_env_ref(env, "Int", derive));

    let base = &int_type!();
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part2() {
    let env = &env();
    let derive = &Type::TypeEnvRef("A".to_string());
    assert!(lift_env_ref(env, "Int", derive));

    let base = &int_type!();
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part3() {
    let env = &env();
    let derive = &Type::TypeEnvRef("B".to_string());
    assert!(lift_env_ref(env, "Int", derive));

    let base = &int_type!();
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part4() {
    let env = &env();
    let derive = &Type::TypeEnvRef("C".to_string());
    assert!(lift_env_ref(env, "Int", derive));

    let base = &int_type!();
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part5() {
    let env = &env();
    let derive = &Type::TypeEnvRef("D".to_string());
    assert!(!lift_env_ref(env, "Int", derive));

    let base = &int_type!();
    assert!(!lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), None);
}

#[test]
fn test_lift_part6() {
    let env = &env();
    let derive = &Type::TypeEnvRef("S0".to_string());
    assert!(lift_env_ref(env, "Int", derive));

    let base = &int_type!();
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part7() {
    let env = &env();
    let derive = &Type::TypeEnvRef("S1".to_string());
    assert!(lift_env_ref(env, "Int", derive));

    let base = &int_type!();
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part8() {
    let env = &env();
    let derive = &Type::TypeEnvRef("S2".to_string());
    assert!(lift_env_ref(env, "Int", derive));

    let base = &int_type!();
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part9() {
    let env = &env();
    let derive = &Type::TypeEnvRef("S3".to_string());
    assert!(!lift_env_ref(env, "Int", derive));

    let base = &int_type!();
    assert!(!lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), None);
}
