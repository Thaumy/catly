use crate::btree_set;
use crate::parser::r#type::Type;
use crate::unifier::int::lift;

fn env() -> Vec<(String, Type)> {
    /* env:
    type None = Int
    type Nothing = None
    type A = Unit
    type IntOrA = Int | A
    type UnitOrA = Unit | A
    type NothingOrA = Nothing | A
    */
    vec![
        ("None".to_string(),
         Type::TypeEnvRef("Int".to_string())),
        ("Nothing".to_string(),
         Type::TypeEnvRef("None".to_string())),
        ("A".to_string(),
         Type::TypeEnvRef("Unit".to_string())),
        ("IntOrA".to_string(),
         Type::SumType(btree_set![
                Type::TypeEnvRef("Int".to_string()),
                Type::TypeEnvRef("A".to_string()),
            ])),
        ("UnitOrA".to_string(),
         Type::SumType(btree_set![
                Type::TypeEnvRef("Unit".to_string()),
                Type::TypeEnvRef("A".to_string()),
            ])),
        ("NothingOrA".to_string(),
         Type::SumType(btree_set![
                Type::TypeEnvRef("Nothing".to_string()),
                Type::TypeEnvRef("A".to_string()),
            ])),
    ]
}

#[test]
fn test_lift_part1() {
    let env = &env();
    let derive = &Type::TypeEnvRef("Int".to_string());

    assert!(lift(env, derive));
}

#[test]
fn test_lift_part2() {
    let env = &env();
    let derive = &Type::TypeEnvRef("None".to_string());

    assert!(lift(env, derive));
}

#[test]
fn test_lift_part3() {
    let env = &env();
    let derive = &Type::TypeEnvRef("Nothing".to_string());

    assert!(lift(env, derive));
}

#[test]
fn test_lift_part4() {
    let env = &env();
    let derive = &Type::TypeEnvRef("A".to_string());

    assert!(!lift(env, derive));
}

#[test]
fn test_lift_part5() {
    let env = &env();
    let derive = &Type::TypeEnvRef("IntOrA".to_string());

    assert!(lift(env, derive));
}

#[test]
fn test_lift_part6() {
    let env = &env();
    let derive = &Type::TypeEnvRef("UnitOrA".to_string());

    assert!(!lift(env, derive));
}

#[test]
fn test_lift_part7() {
    let env = &env();
    let derive = &Type::TypeEnvRef("NothingOrA".to_string());

    assert!(lift(env, derive));
}
