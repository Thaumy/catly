use crate::btree_set;
use crate::infra::option::AnyExt;
use crate::parser::r#type::Type;
use crate::unifier::env_ref::lift as lift_env_ref;
use crate::unifier::lift;
use crate::unifier::unify;

fn env() -> Vec<(String, Type)> {
    /* env:
    type A = X
    type B = A

    type S0 = D | X
    */
    vec![
        ("A".to_string(), Type::TypeEnvRef("X".to_string())),
        ("B".to_string(), Type::TypeEnvRef("A".to_string())),
        (
            "S0".to_string(),
            Type::SumType(btree_set![
                Type::TypeEnvRef("D".to_string()),
                Type::TypeEnvRef("X".to_string()),
            ])
        ),
    ]
}

#[test]
fn test_lift_part1() {
    let env = &env();
    let derive = &Type::TypeEnvRef("Discard".to_string());
    assert!(lift_env_ref(env, "Discard", derive));

    let base = &Type::TypeEnvRef("Discard".to_string());
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part2() {
    let env = &env();
    let derive = &Type::TypeEnvRef("A".to_string());
    assert!(lift_env_ref(env, "Discard", derive));

    let base = &Type::TypeEnvRef("Discard".to_string());
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part3() {
    let env = &env();
    let derive = &Type::TypeEnvRef("B".to_string());
    assert!(lift_env_ref(env, "Discard", derive));

    let base = &Type::TypeEnvRef("Discard".to_string());
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part6() {
    let env = &env();
    let derive = &Type::TypeEnvRef("S0".to_string());
    assert!(lift_env_ref(env, "Discard", derive));

    let base = &Type::TypeEnvRef("Discard".to_string());
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}
