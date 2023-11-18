use crate::btree_set;
use crate::infer::env::bool_type;
use crate::infer::env::false_type;
use crate::infer::env::int_type;
use crate::infer::env::namely_type;
use crate::infer::env::sum_type;
use crate::infer::env::true_type;
use crate::infer::env::type_env::TypeEnv;
use crate::infra::option::WrapOption;
use crate::unify::namely::lift_namely;

fn env<'t>() -> TypeEnv<'t> {
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
fn test_part1() {
    let env = &env();
    let derive = &namely_type!("A");
    assert!(lift_namely(env, "A", derive).is_some());

    let base = &namely_type!("A");
    assert!(base
        .lift_to(env, derive)
        .is_some());
    assert_eq!(base.unify(env, derive), derive.clone().wrap_some());
}

#[test]
fn test_part2() {
    let env = &env();
    let derive = &namely_type!("B");
    assert!(!lift_namely(env, "A", derive).is_some());

    let base = &namely_type!("A");
    assert!(!base
        .lift_to(env, derive)
        .is_some());
    assert_eq!(base.unify(env, derive), None);
}

#[test]
fn test_part3() {
    let env = &env();
    let derive = &sum_type![namely_type!("A"), namely_type!("B")];
    assert!(lift_namely(env, "A", derive).is_some());

    let base = &namely_type!("A");
    assert!(base
        .lift_to(env, derive)
        .is_some());
    assert_eq!(base.unify(env, derive), derive.clone().wrap_some());
}

#[test]
fn test_part4() {
    let env = &env();
    let derive = &bool_type!();

    assert!(lift_namely(env, "True", derive).is_some());

    let base = &namely_type!("True");
    assert!(base
        .lift_to(env, derive)
        .is_some());
    assert_eq!(base.unify(env, derive), derive.clone().wrap_some());
}
