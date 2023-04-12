use crate::env::type_env::TypeEnv;
use crate::infra::option::AnyExt;
use crate::parser::r#type::Type;
use crate::unifier::env_ref::lift as lift_env_ref;
use crate::unifier::lift;
use crate::unifier::unify;
use crate::{btree_set, namely_type, unit_type};

fn env() -> TypeEnv {
    /* env:
    type A = Unit
    type B = A
    type C = B

    type S0 = D | Unit
    type S1 = D | A
    type S2 = D | C
    type S3 = D | E
    */
    let vec = vec![
        ("A".to_string(), unit_type!()),
        ("B".to_string(), namely_type!("A")),
        ("C".to_string(), namely_type!("B")),
        (
            "S0".to_string(),
            Type::SumType(btree_set![
                namely_type!("D"),
                unit_type!(),
            ])
        ),
        (
            "S1".to_string(),
            Type::SumType(btree_set![
                namely_type!("D"),
                namely_type!("A"),
            ])
        ),
        (
            "S2".to_string(),
            Type::SumType(btree_set![
                namely_type!("D"),
                namely_type!("C"),
            ])
        ),
        (
            "S3".to_string(),
            Type::SumType(btree_set![
                namely_type!("D"),
                namely_type!("E"),
            ])
        ),
    ];

    TypeEnv::new(vec)
}

#[test]
fn test_lift_part1() {
    let env = &env();
    let derive = &unit_type!();
    assert!(lift_env_ref(env, "Unit", derive));

    let base = &unit_type!();
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part2() {
    let env = &env();
    let derive = &namely_type!("A");
    assert!(lift_env_ref(env, "Unit", derive));

    let base = &unit_type!();
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part3() {
    let env = &env();
    let derive = &namely_type!("B");
    assert!(lift_env_ref(env, "Unit", derive));

    let base = &unit_type!();
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part4() {
    let env = &env();
    let derive = &namely_type!("C");
    assert!(lift_env_ref(env, "Unit", derive));

    let base = &unit_type!();
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part5() {
    let env = &env();
    let derive = &namely_type!("D");
    assert!(!lift_env_ref(env, "Unit", derive));

    let base = &unit_type!();
    assert!(!lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), None);
}

#[test]
fn test_lift_part6() {
    let env = &env();
    let derive = &namely_type!("S0");
    assert!(lift_env_ref(env, "Unit", derive));

    let base = &unit_type!();
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part7() {
    let env = &env();
    let derive = &namely_type!("S1");
    assert!(lift_env_ref(env, "Unit", derive));

    let base = &unit_type!();
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part8() {
    let env = &env();
    let derive = &namely_type!("S2");
    assert!(lift_env_ref(env, "Unit", derive));

    let base = &unit_type!();
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part9() {
    let env = &env();
    let derive = &namely_type!("S3");
    assert!(!lift_env_ref(env, "Unit", derive));

    let base = &unit_type!();
    assert!(!lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), None);
}
