use crate::env::type_env::TypeEnv;
use crate::infra::option::AnyExt;
use crate::unifier::lift;
use crate::unifier::namely::lift as lift_namely;
use crate::unifier::unify;
use crate::{btree_set, namely_type, sum_type, unit_type};

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
        ("S0".to_string(), sum_type![
            namely_type!("D"),
            unit_type!(),
        ]),
        ("S1".to_string(), sum_type![
            namely_type!("D"),
            namely_type!("A"),
        ]),
        ("S2".to_string(), sum_type![
            namely_type!("D"),
            namely_type!("C"),
        ]),
        ("S3".to_string(), sum_type![
            namely_type!("D"),
            namely_type!("E"),
        ]),
    ];

    TypeEnv::new(vec)
}

#[test]
fn test_part1() {
    let env = &env();
    let derive = &unit_type!();
    assert!(lift_namely(env, "Unit", derive));

    let base = &unit_type!();
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_part2() {
    let env = &env();
    let derive = &namely_type!("A");
    assert!(lift_namely(env, "Unit", derive));

    let base = &unit_type!();
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_part3() {
    let env = &env();
    let derive = &namely_type!("B");
    assert!(lift_namely(env, "Unit", derive));

    let base = &unit_type!();
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_part4() {
    let env = &env();
    let derive = &namely_type!("C");
    assert!(lift_namely(env, "Unit", derive));

    let base = &unit_type!();
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_part5() {
    let env = &env();
    let derive = &namely_type!("D");
    assert!(!lift_namely(env, "Unit", derive));

    let base = &unit_type!();
    assert!(!lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), None);
}

#[test]
fn test_part6() {
    let env = &env();
    let derive = &namely_type!("S0");
    assert!(lift_namely(env, "Unit", derive));

    let base = &unit_type!();
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_part7() {
    let env = &env();
    let derive = &namely_type!("S1");
    assert!(lift_namely(env, "Unit", derive));

    let base = &unit_type!();
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_part8() {
    let env = &env();
    let derive = &namely_type!("S2");
    assert!(lift_namely(env, "Unit", derive));

    let base = &unit_type!();
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_part9() {
    let env = &env();
    let derive = &namely_type!("S3");
    assert!(!lift_namely(env, "Unit", derive));

    let base = &unit_type!();
    assert!(!lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), None);
}
