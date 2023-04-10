use crate::btree_set;
use crate::infra::option::AnyExt;
use crate::parser::r#type::Type;
use crate::type_checker::env::type_env::TypeEnv;
use crate::unifier::env_ref::lift as lift_env_ref;
use crate::unifier::lift;
use crate::unifier::unify;

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
        ("A".to_string(), Type::TypeEnvRef("Int".to_string())),
        ("B".to_string(), Type::TypeEnvRef("A".to_string())),
        ("C".to_string(), Type::TypeEnvRef("B".to_string())),
        (
            "AB".to_string(),
            Type::SumType(btree_set![
                Type::TypeEnvRef("A".to_string()),
                Type::TypeEnvRef("B".to_string()),
            ])
        ),
        ("True".to_string(), Type::TypeEnvRef("Int".to_string())),
        ("False".to_string(), Type::TypeEnvRef("Int".to_string())),
        (
            "Bool".to_string(),
            Type::SumType(btree_set![
                Type::TypeEnvRef("True".to_string()),
                Type::TypeEnvRef("False".to_string()),
            ])
        ),
    ];

    TypeEnv::new(vec)
}

#[test]
fn test_lift_part1() {
    let env = &env();
    let derive = &Type::TypeEnvRef("A".to_string());
    assert!(lift_env_ref(env, "A", derive));

    let base = &Type::TypeEnvRef("A".to_string());
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part2() {
    let env = &env();
    let derive = &Type::TypeEnvRef("B".to_string());
    assert!(!lift_env_ref(env, "A", derive));

    let base = &Type::TypeEnvRef("A".to_string());
    assert!(!lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), None);
}

#[test]
fn test_lift_part3() {
    let env = &env();
    let derive = &Type::SumType(btree_set![
        Type::TypeEnvRef("A".to_string()),
        Type::TypeEnvRef("B".to_string()),
    ]);
    assert!(lift_env_ref(env, "A", derive));

    let base = &Type::TypeEnvRef("A".to_string());
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part4() {
    let env = &env();
    let derive = &Type::TypeEnvRef("Bool".to_string());

    assert!(lift_env_ref(env, "True", derive));

    let base = &Type::TypeEnvRef("True".to_string());
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}
