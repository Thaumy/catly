use crate::btree_set;
use crate::infer::env::closure_type;
use crate::infer::env::int_type;
use crate::infer::env::namely_type;
use crate::infer::env::prod_type;
use crate::infer::env::sum_type;
use crate::infer::env::unit_type;
use crate::infra::WrapOption;
use crate::infra::WrapRc;
use crate::parser::ast::test::f;
use crate::parser::define::Define;
use crate::parser::expr::r#type::Expr;

#[test]
fn test_part2() {
    let t1 = Define::TypeDef("Foo".to_string(), prod_type![
        ("abc".to_string(), namely_type!("A")),
        ("uuu".to_string(), namely_type!("IntList")),
        ("intList".to_string(), prod_type![
            ("x".to_string(), namely_type!("X")),
            ("y".to_string(), namely_type!("Y")),
        ]),
    ]);
    let d1 = Define::ExprDef(
        "bar".to_string(),
        None,
        Expr::Match(
            None,
            Expr::EnvRef(None, "x".to_string()).wrap_rc(),
            vec![
                (Expr::Int(None, 1), Expr::Cond(None, Expr::EnvRef(None, "a".to_string()).wrap_rc(), Expr::EnvRef(None, "b".to_string()).wrap_rc(), Expr::EnvRef(None, "c".to_string()).wrap_rc())),
                (Expr::EnvRef(None, "v".to_string()), Expr::Closure(None, "a".to_string().wrap_some(), None, Expr::Closure(None, "b".to_string().wrap_some(), None, Expr::Apply(None, Expr::Apply(None, Expr::EnvRef(None, "add".to_string()).wrap_rc(), Expr::EnvRef(None, "a".to_string()).wrap_rc()).wrap_rc(), Expr::EnvRef(None, "b".to_string()).wrap_rc()).wrap_rc()).wrap_rc())),
                (Expr::Struct(None, vec![("a".to_string(), None, Expr::Discard(None)), ("b".to_string(), None, Expr::Struct(None, vec![("foo".to_string(), None, Expr::Discard(None)), ("bar".to_string(), None, Expr::Discard(None))])), ("c".to_string(), None, Expr::Int(None, 3))]), Expr::Struct(None, vec![("x".to_string(), None, Expr::Int(None, 123)), ("y".to_string(), None, Expr::EnvRef(None, "c".to_string()))])),
                (
                    Expr::Discard(None),
                    Expr::Match(
                        None,
                        Expr::EnvRef(None, "y".to_string()).wrap_rc(),
                        vec![
                            (Expr::Int(None, 1), Expr::Unit(None)),
                            (
                                Expr::Unit(None),
                                Expr::Closure(
                                    None,
                                    "a".to_string().wrap_some(),
                                    None,
                                    Expr::Closure(
                                        None,
                                        "b".to_string().wrap_some(),
                                        None,
                                        Expr::Match(
                                            None,
                                            Expr::EnvRef(None, "z".to_string()).wrap_rc(),
                                            vec![(Expr::Discard(None), Expr::Int(None, 114514)), (Expr::EnvRef(None, "a".to_string()), Expr::Closure(None, "x".to_string().wrap_some(), None, Expr::Closure(None, "y".to_string().wrap_some(), None, Expr::Apply(None, Expr::Apply(None, Expr::EnvRef(None, "add".to_string()).wrap_rc(), Expr::Unit(None).wrap_rc()).wrap_rc(), Expr::EnvRef(None, "y".to_string()).wrap_rc()).wrap_rc()).wrap_rc()))],
                                        )
                                        .wrap_rc(),
                                    )
                                    .wrap_rc(),
                                ),
                            ),
                            (Expr::Discard(None), Expr::EnvRef(None, "baz".to_string())),
                        ],
                    ),
                ),
            ],
        ),
    );
    let t2 = Define::TypeDef("Love".to_string(), sum_type![
        namely_type!("A"),
        unit_type!(),
        namely_type!("C"),
        int_type!(),
    ]);
    let i1 = Define::ExprDef(
        "i".to_string(),
        int_type!().wrap_some(),
        Expr::Int(int_type!().wrap_some(), 0)
    );
    let d2 = Define::ExprDef(
        "main".to_string(),
        closure_type!(unit_type!(), unit_type!()).wrap_some(),
        Expr::Let(
            None,
            false,
            "a".to_string(),
            None,
            Expr::Int(None, 123).wrap_rc(),
            Expr::Let(
                None,
                false,
                "b".to_string(),
                None,
                Expr::Let(
                    None,
                    false,
                    "x".to_string(),
                    None,
                    Expr::Closure(
                        None,
                        "i".to_string().wrap_some(),
                        None,
                        Expr::Closure(
                            None,
                            "j".to_string().wrap_some(),
                            None,
                            Expr::EnvRef(None, "k".to_string())
                                .wrap_rc()
                        )
                        .wrap_rc()
                    )
                    .wrap_rc(),
                    Expr::Let(
                        None,
                        false,
                        "y".to_string(),
                        None,
                        Expr::EnvRef(None, "a".to_string()).wrap_rc(),
                        Expr::Let(
                            None,
                            false,
                            "z".to_string(),
                            None,
                            Expr::Unit(None).wrap_rc(),
                            Expr::EnvRef(None, "a".to_string())
                                .wrap_rc()
                        )
                        .wrap_rc()
                    )
                    .wrap_rc()
                )
                .wrap_rc(),
                Expr::Let(
                    None,
                    false,
                    "d".to_string(),
                    None,
                    Expr::Apply(
                        None,
                        Expr::EnvRef(None, "neg".to_string())
                            .wrap_rc(),
                        Expr::Int(None, 1).wrap_rc()
                    )
                    .wrap_rc(),
                    Expr::Let(
                        None,
                        false,
                        "e".to_string(),
                        None,
                        Expr::Int(None, 6).wrap_rc(),
                        Expr::Let(
                            None,
                            false,
                            "k".to_string(),
                            None,
                            Expr::Unit(None).wrap_rc(),
                            Expr::Let(
                                None,
                                false,
                                "m".to_string(),
                                None,
                                Expr::Unit(None).wrap_rc(),
                                Expr::Let(
                                    None,
                                    false,
                                    "n".to_string(),
                                    None,
                                    Expr::Int(None, 4).wrap_rc(),
                                    Expr::Apply(
                                        None,
                                        Expr::Apply(
                                            None,
                                            Expr::EnvRef(
                                                None,
                                                "add".to_string()
                                            )
                                            .wrap_rc(),
                                            Expr::Unit(None)
                                                .wrap_rc()
                                        )
                                        .wrap_rc(),
                                        Expr::Int(None, 456)
                                            .wrap_rc()
                                    )
                                    .wrap_rc()
                                )
                                .wrap_rc()
                            )
                            .wrap_rc()
                        )
                        .wrap_rc()
                    )
                    .wrap_rc()
                )
                .wrap_rc()
            )
            .wrap_rc()
        )
    );
    let r = vec![t1, d1, t2, i1, d2];
    let r = Some(r);

    let seq =
        "type Foo = { abc: A, uuu: IntList, intList: { x: X, y: Y } }
         def bar =
             match x with
             | 1 -> if a then b else c
             | v -> (a -> b -> add a b)
             | { a = _, b = { foo = _, bar = _ }, c = 3 } ->
                 { x = 123, y = c }
             | _ ->
                 match y with
                 | 1 -> ()
                 | () ->
                     (a -> b ->
                         match z with
                         | _ -> 114514
                         | a -> (x -> y -> add () y))
                 | _ -> baz
         type Love = A | Unit | C | Int
         def i: Int = 0: Int
         def main: Unit -> Unit =
             let a = 123,
                 b =
                     let x = i -> j -> k,
                         y = a
                     in
                     let z = ()
                     in a,
                 d = neg 1
             in
                 let e = 6,
                     k = ()
                 in
                 let m = (),
                     n = 4
                 in
                 add () 456";
    assert_eq!(f(seq), r);
}
