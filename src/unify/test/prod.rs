use crate::btree_set;
use crate::infer::env::namely_type;
use crate::infer::env::prod_type;
use crate::infer::env::sum_type;
use crate::infer::env::TypeEnv;
use crate::infra::WrapOption;
use crate::parser::r#type::Type;
use crate::unify::prod::lift_prod;

fn env<'t>() -> TypeEnv<'t> {
    /* env:
    type S = { a: A }
    type SA = S | A
    type BA = B | A
    */
    let vec = vec![
        ("S".to_string(), prod_type![(
            "a".to_string(),
            namely_type!("A")
        )]),
        ("SA".to_string(), sum_type![
            namely_type!("S"),
            namely_type!("A"),
        ]),
        ("BA".to_string(), sum_type![
            namely_type!("B"),
            namely_type!("A"),
        ]),
    ];

    TypeEnv::new(vec)
}

#[test]
fn test_part1() {
    let env = &env();
    let v = &vec![("a".to_string(), namely_type!("A"))];
    let derive = &prod_type![("a".to_string(), namely_type!("A"))];
    assert!(lift_prod(env, v, derive).is_some());

    let base = &Type::ProdType(v.clone());
    assert!(base
        .lift_to(env, derive)
        .is_some());
    assert_eq!(base.unify(env, derive), derive.clone().wrap_some());
}

#[test]
fn test_part2() {
    let env = &env();
    let v = &vec![("a".to_string(), namely_type!("A"))];
    let derive = &namely_type!("S");
    assert!(lift_prod(env, v, derive).is_some());

    let base = &Type::ProdType(v.clone());
    assert!(base
        .lift_to(env, derive)
        .is_some());
    assert_eq!(base.unify(env, derive), derive.clone().wrap_some());
}

#[test]
fn test_part3() {
    let env = &env();
    let v = &vec![("a".to_string(), namely_type!("A"))];
    let derive = &namely_type!("SA");
    assert!(lift_prod(env, v, derive).is_none());

    let base = &Type::ProdType(v.clone());
    assert!(base
        .lift_to(env, derive)
        .is_none());
    assert_eq!(base.unify(env, derive), None);
}

#[test]
fn test_part4() {
    let env = &env();
    let v = &vec![("a".to_string(), namely_type!("A"))];
    let derive = &namely_type!("BA");
    assert!(lift_prod(env, v, derive).is_none());

    let base = &Type::ProdType(v.clone());
    assert!(!base
        .lift_to(env, derive)
        .is_some());
    assert_eq!(base.unify(env, derive), None);
}
