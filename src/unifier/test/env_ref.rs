use crate::env::type_env::TypeEnv;
use crate::infra::option::AnyExt;
use crate::unifier::env_ref::lift as lift_env_ref;
use crate::unifier::lift;
use crate::unifier::unify;
use crate::{
    bool_type,
    btree_set,
    false_type,
    int_type,
    namely_type,
    sum_type,
    true_type
};

fn env() -> TypeEnv {
    /* env:
    type A = Int
    type B = A
    type C = B

    type True = Int
    type False = Int
    type Bool = True | False
    */

    let vec = vec![
        ("A".to_string(), int_type!()),
        ("B".to_string(), namely_type!("A")),
        ("C".to_string(), namely_type!("B")),
        ("AB".to_string(), sum_type![
            namely_type!("A"),
            namely_type!("B"),
        ]),
        ("True".to_string(), int_type!()),
        ("False".to_string(), int_type!()),
        ("Bool".to_string(), sum_type![true_type!(), false_type!()]),
    ];

    TypeEnv::new(vec)
}

#[test]
fn test_lift_part1() {
    let env = &env();
    let derive = &namely_type!("A");
    assert!(lift_env_ref(env, "A", derive));

    let base = &namely_type!("A");
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part2() {
    let env = &env();
    let derive = &namely_type!("B");
    assert!(!lift_env_ref(env, "A", derive));

    let base = &namely_type!("A");
    assert!(!lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), None);
}

#[test]
fn test_lift_part3() {
    let env = &env();
    let derive = &sum_type![namely_type!("A"), namely_type!("B")];
    assert!(lift_env_ref(env, "A", derive));

    let base = &namely_type!("A");
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part4() {
    let env = &env();
    let derive = &bool_type!();

    assert!(lift_env_ref(env, "True", derive));

    let base = &namely_type!("True");
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}
