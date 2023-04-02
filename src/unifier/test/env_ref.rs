use crate::btree_set;
use crate::parser::infra::option::AnyExt;
use crate::parser::r#type::Type;
use crate::unifier::env_ref::lift as lift_env_ref;
use crate::unifier::lift;
use crate::unifier::unify;

fn env() -> Vec<(String, Type)> {
    /* env:
    type B = A
    type C = B
    type AB = A | B
    */

    vec![
        ("B".to_string(),
         Type::TypeEnvRef("A".to_string())),
        ("C".to_string(),
         Type::TypeEnvRef("B".to_string())),
        ("AB".to_string(),
         Type::SumType(btree_set![
            Type::TypeEnvRef("A".to_string()),
            Type::TypeEnvRef("B".to_string()),
        ])),
    ]
}

#[test]
fn test_lift_part1() {
    let env = &env();
    let derive = &Type::TypeEnvRef("A".to_string());
    assert!(lift_env_ref(env, "A", derive));

    let base = &Type::TypeEnvRef("A".to_string());
    assert!(lift(env, base, derive));
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part2() {
    let env = &env();
    let derive = &Type::TypeEnvRef("B".to_string());
    assert!(!lift_env_ref(env, "A", derive));

    let base = &Type::TypeEnvRef("A".to_string());
    assert!(!lift(env, base, derive));
    assert_eq!(unify(env, base, derive), None);
}

#[test]
fn test_lift_part3() {
    let env = &env();
    let derive = &Type::TypeEnvRef("AB".to_string());
    assert!(!lift_env_ref(env, "A", derive));

    let base = &Type::TypeEnvRef("A".to_string());
    assert!(!lift(env, base, derive));
    assert_eq!(unify(env, base, derive), None);
}

#[test]
fn test_lift_part4() {
    let env = &env();
    let derive = &Type::SumType(btree_set![
        Type::TypeEnvRef("A".to_string()),
        Type::TypeEnvRef("B".to_string()),
    ]);
    assert!(lift_env_ref(env, "A", derive));

    let base = &Type::TypeEnvRef("A".to_string());
    assert!(lift(env, base, derive));
    assert_eq!(unify(env, base, derive), derive.clone().some());
}
