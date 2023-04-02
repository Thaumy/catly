use crate::btree_set;
use crate::parser::infra::r#box::Ext;
use crate::parser::r#type::Type;
use crate::unifier::closure::lift;

fn env() -> Vec<(String, Type)> {
    /* env:
    type F = A -> B
    type G = A -> B -> C
    type FG = F | G
    type AB = A | B
    */
    vec![
        ("F".to_string(),
         Type::ClosureType(
             Type::TypeEnvRef("A".to_string()).boxed(),
             Type::TypeEnvRef("B".to_string()).boxed(),
         )),
        ("G".to_string(),
         Type::ClosureType(
             Type::TypeEnvRef("A".to_string()).boxed(),
             Type::ClosureType(
                 Type::TypeEnvRef("B".to_string()).boxed(),
                 Type::TypeEnvRef("C".to_string()).boxed(),
             ).boxed(),
         )),
        ("FG".to_string(),
         Type::SumType(btree_set![
            Type::TypeEnvRef("F".to_string()),
            Type::TypeEnvRef("G".to_string()),
        ])),
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
    let a = &Type::TypeEnvRef("A".to_string());
    let b = &Type::TypeEnvRef("B".to_string());
    let derive = &Type::ClosureType(
        Type::TypeEnvRef("A".to_string()).boxed(),
        Type::TypeEnvRef("B".to_string()).boxed(),
    );

    assert!(lift(env, a, b, derive));
}

#[test]
fn test_lift_part2() {
    let env = &env();
    let a = &Type::TypeEnvRef("A".to_string());
    let b = &Type::TypeEnvRef("B".to_string());
    let derive = &Type::TypeEnvRef("F".to_string());

    assert!(lift(env, a, b, derive));
}

#[test]
fn test_lift_part3() {
    let env = &env();
    let a = &Type::TypeEnvRef("A".to_string());
    let b = &Type::TypeEnvRef("B".to_string());
    let derive = &Type::TypeEnvRef("G".to_string());

    assert!(!lift(env, a, b, derive));
}

#[test]
fn test_lift_part4() {
    let env = &env();
    let a = &Type::TypeEnvRef("A".to_string());
    let b = &Type::TypeEnvRef("B".to_string());
    let derive = &Type::TypeEnvRef("FG".to_string());

    assert!(lift(env, a, b, derive));
}

#[test]
fn test_lift_part5() {
    let env = &env();
    let a = &Type::TypeEnvRef("A".to_string());
    let b = &Type::TypeEnvRef("B".to_string());
    let derive = &Type::TypeEnvRef("AB".to_string());

    assert!(!lift(env, a, b, derive));
}
