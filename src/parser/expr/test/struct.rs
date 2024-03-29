use crate::btree_set;
use crate::infer::env::closure_type;
use crate::infer::env::int_type;
use crate::infer::env::namely_type;
use crate::infer::env::prod_type;
use crate::infer::env::sum_type;
use crate::infer::env::unit_type;
use crate::infra::WrapOption;
use crate::infra::WrapRc;
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
                    Expr::Int(None, 123).wrap_rc(),
                    Expr::Unit(None).wrap_rc(),
                    Expr::Int(None, 0).wrap_rc()
                )
            )])
        ),
        ("x".to_string(), None, Expr::Int(None, 1)),
    ]);
    let fun = Expr::Closure(
        None,
        "x".to_string().wrap_some(),
        None,
        Expr::Closure(
            None,
            "y".to_string().wrap_some(),
            None,
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "add".to_string()).wrap_rc(),
                    Expr::EnvRef(None, "x".to_string()).wrap_rc()
                )
                .wrap_rc(),
                Expr::EnvRef(None, "y".to_string()).wrap_rc()
            )
            .wrap_rc()
        )
        .wrap_rc()
    );
    let r = Expr::Struct(None, vec![
        ("a".to_string(), None, a),
        (
            "ab".to_string(),
            None,
            Expr::Apply(
                None,
                Expr::EnvRef(None, "neg".to_string()).wrap_rc(),
                Expr::Int(None, 1).wrap_rc()
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
            int_type!().wrap_some(),
            Expr::Struct(None, vec![(
                "efg".to_string(),
                None,
                Expr::Cond(
                    None,
                    Expr::Int(None, 123).wrap_rc(),
                    Expr::Unit(None).wrap_rc(),
                    Expr::Int(None, 0).wrap_rc()
                )
            )])
        ),
        ("x".to_string(), None, Expr::Int(None, 1)),
    ]);
    let fun = Expr::Closure(
        None,
        "x".to_string().wrap_some(),
        None,
        Expr::Closure(
            None,
            "y".to_string().wrap_some(),
            None,
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "add".to_string()).wrap_rc(),
                    Expr::EnvRef(None, "x".to_string()).wrap_rc()
                )
                .wrap_rc(),
                Expr::EnvRef(None, "y".to_string()).wrap_rc()
            )
            .wrap_rc()
        )
        .wrap_rc()
    );
    let r = Expr::Struct(int_type!().wrap_some(), vec![
        ("a".to_string(), int_type!().wrap_some(), a),
        (
            "ab".to_string(),
            closure_type!(int_type!(), int_type!()).wrap_some(),
            Expr::Apply(
                None,
                Expr::EnvRef(None, "neg".to_string()).wrap_rc(),
                Expr::Int(None, 1).wrap_rc()
            )
        ),
        (
            "fun".to_string(),
            prod_type![
                ("a".to_string(), int_type!()),
                ("b".to_string(), unit_type!()),
            ]
            .wrap_some(),
            fun
        ),
        (
            "y".to_string(),
            prod_type![("a".to_string(), prod_type![
                ("a".to_string(), int_type!()),
                ("b".to_string(), unit_type!()),
            ]),]
            .wrap_some(),
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
    let ab = prod_type![("a".to_string(), int_type!())].wrap_some();

    let cd = prod_type![
        ("a".to_string(), int_type!()),
        ("b".to_string(), int_type!()),
    ]
    .wrap_some();

    let ef = prod_type![
        ("a".to_string(), int_type!()),
        ("b".to_string(), int_type!()),
        ("c".to_string(), int_type!()),
    ]
    .wrap_some();

    let r = Expr::Struct(None, vec![
        (
            "a".to_string(),
            ab.clone(),
            Expr::Unit(unit_type!().wrap_some())
        ),
        (
            "b".to_string(),
            ab.clone(),
            Expr::Unit(unit_type!().wrap_some())
        ),
        (
            "c".to_string(),
            cd.clone(),
            Expr::Unit(unit_type!().wrap_some())
        ),
        (
            "d".to_string(),
            cd.clone(),
            Expr::Unit(unit_type!().wrap_some())
        ),
        (
            "e".to_string(),
            ef.clone(),
            Expr::Unit(unit_type!().wrap_some())
        ),
        (
            "f".to_string(),
            ef.clone(),
            Expr::Unit(unit_type!().wrap_some())
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
            .wrap_some(),
            Expr::Unit(unit_type!().wrap_some())
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
