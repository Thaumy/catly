use crate::btree_set;
use crate::infer::env::r#macro::closure_type;
use crate::infer::env::r#macro::int_type;
use crate::infer::env::r#macro::namely_type;
use crate::infer::env::r#macro::prod_type;
use crate::infer::env::r#macro::sum_type;
use crate::infer::env::r#macro::unit_type;
use crate::infra::option::OptionAnyExt;
use crate::infra::rc::RcAnyExt;
use crate::parser::expr::r#type::Expr;
use crate::parser::expr::test::f;

#[test]
fn test_part1() {
    let r = Expr::Struct(None, vec![
        ("a".to_string(), None, Expr::Int(None, 123)),
        (
            "ab".to_string(),
            None,
            Expr::EnvRef(None, "ref".to_string())
        ),
        ("abc".to_string(), None, Expr::Unit(None)),
    ]);
    let r = Some(r);

    let seq = "{ a = 123, ab = ref, abc = () }";
    assert_eq!(f(seq), r);
    let seq = "{ a = 123, ab = ref, abc = (),}";
    assert_eq!(f(seq), r);
    let seq = "(({ a = (((123))), ab = (((ref))), abc = ((())) }))";
    assert_eq!(f(seq), r);
    let seq = "(({ a = (((123))), ab = (((ref))), abc = ((())),}))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_part2() {
    let a = Expr::Struct(None, vec![
        (
            "abc".to_string(),
            None,
            Expr::Struct(None, vec![(
                "efg".to_string(),
                None,
                Expr::Cond(
                    None,
                    Expr::Int(None, 123).rc(),
                    Expr::Unit(None).rc(),
                    Expr::Int(None, 0).rc()
                )
            )])
        ),
        ("x".to_string(), None, Expr::Int(None, 1)),
    ]);
    let fun = Expr::Closure(
        None,
        "x".to_string().some(),
        None,
        Expr::Closure(
            None,
            "y".to_string().some(),
            None,
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "add".to_string()).rc(),
                    Expr::EnvRef(None, "x".to_string()).rc()
                )
                .rc(),
                Expr::EnvRef(None, "y".to_string()).rc()
            )
            .rc()
        )
        .rc()
    );
    let r = Expr::Struct(None, vec![
        ("a".to_string(), None, a),
        (
            "ab".to_string(),
            None,
            Expr::Apply(
                None,
                Expr::EnvRef(None, "neg".to_string()).rc(),
                Expr::Int(None, 1).rc()
            )
        ),
        ("fun".to_string(), None, fun),
    ]);
    let r = Some(r);

    let seq = "{ \
               a = { abc = { efg = if 123 then () else 0 }, x = 1 }, \
               ab = neg 1, \
               fun = x -> y -> add x y \
             }";
    assert_eq!(f(seq), r);
    let seq = "((({ \
                  a = ((({ abc = { efg = if 123 then ((())) else 0 }, x = 1 }))), \
                  ab = (((neg))) 1, \
                  fun = (x -> y -> add x y) \
            })))";
    assert_eq!(f(seq), r);
    let seq = "((({ \
                  (((a))) = ((({ abc = { efg = if (((123))) then ((())) else 0 }, x = (((1))) }))), \
                  (((ab))) = ((((((neg))) (((1)))))), \
                  (((fun))) = (x -> (((y -> add x y)))) \
            })))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_part3() {
    let a = Expr::Struct(None, vec![
        (
            "abc".to_string(),
            int_type!().some(),
            Expr::Struct(None, vec![(
                "efg".to_string(),
                None,
                Expr::Cond(
                    None,
                    Expr::Int(None, 123).rc(),
                    Expr::Unit(None).rc(),
                    Expr::Int(None, 0).rc()
                )
            )])
        ),
        ("x".to_string(), None, Expr::Int(None, 1)),
    ]);
    let fun = Expr::Closure(
        None,
        "x".to_string().some(),
        None,
        Expr::Closure(
            None,
            "y".to_string().some(),
            None,
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "add".to_string()).rc(),
                    Expr::EnvRef(None, "x".to_string()).rc()
                )
                .rc(),
                Expr::EnvRef(None, "y".to_string()).rc()
            )
            .rc()
        )
        .rc()
    );
    let r = Expr::Struct(int_type!().some(), vec![
        ("a".to_string(), int_type!().some(), a),
        (
            "ab".to_string(),
            closure_type!(int_type!(), int_type!()).some(),
            Expr::Apply(
                None,
                Expr::EnvRef(None, "neg".to_string()).rc(),
                Expr::Int(None, 1).rc()
            )
        ),
        (
            "fun".to_string(),
            prod_type![
                ("a".to_string(), int_type!()),
                ("b".to_string(), unit_type!()),
            ]
            .some(),
            fun
        ),
        (
            "y".to_string(),
            prod_type![("a".to_string(), prod_type![
                ("a".to_string(), int_type!()),
                ("b".to_string(), unit_type!()),
            ]),]
            .some(),
            Expr::Int(None, 0)
        ),
    ]);
    let r = Some(r);

    let seq = "{ \
           a: Int = { abc: Int = { efg = if 123 then () else 0 }, x = 1 }, \
           ab: Int -> Int = neg 1, \
           fun: { a: Int, b: Unit } = x -> y -> add x y, \
           y: { a: { a: Int, b: Unit,} } = 0 \
         }: Int";
    assert_eq!(f(seq), r);
}

#[test]
fn test_part4() {
    let ab = prod_type![("a".to_string(), int_type!())].some();

    let cd = prod_type![
        ("a".to_string(), int_type!()),
        ("b".to_string(), int_type!()),
    ]
    .some();

    let ef = prod_type![
        ("a".to_string(), int_type!()),
        ("b".to_string(), int_type!()),
        ("c".to_string(), int_type!()),
    ]
    .some();

    let r = Expr::Struct(None, vec![
        (
            "a".to_string(),
            ab.clone(),
            Expr::Unit(unit_type!().some())
        ),
        (
            "b".to_string(),
            ab.clone(),
            Expr::Unit(unit_type!().some())
        ),
        (
            "c".to_string(),
            cd.clone(),
            Expr::Unit(unit_type!().some())
        ),
        (
            "d".to_string(),
            cd.clone(),
            Expr::Unit(unit_type!().some())
        ),
        (
            "e".to_string(),
            ef.clone(),
            Expr::Unit(unit_type!().some())
        ),
        (
            "f".to_string(),
            ef.clone(),
            Expr::Unit(unit_type!().some())
        ),
        (
            "g".to_string(),
            prod_type![
                ("a".to_string(), int_type!()),
                ("b".to_string(), sum_type![
                    namely_type!("A"),
                    namely_type!("B"),
                ]),
                ("c".to_string(), sum_type![
                    namely_type!("A"),
                    namely_type!("B"),
                    namely_type!("C"),
                ]),
            ]
            .some(),
            Expr::Unit(unit_type!().some())
        ),
    ]);
    let r = Some(r);

    let seq = "{ \
           a: { a: Int } = (): Unit, \
           b: { a: Int,} = (): Unit, \
           c: { a: Int, b: Int } = (): Unit, \
           d: { a: Int, b: Int,} = (): Unit, \
           e: { a: Int, b: Int, c: Int } = (): Unit, \
           f: { a: Int, b: Int, c: Int,} = (): Unit, \
           g: { a: Int, b: A | B, c: A | B | C } = (): Unit, \
        }";
    assert_eq!(f(seq), r);
}
