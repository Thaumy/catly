use crate::btree_set;
use crate::infer::env::int_type;
use crate::infer::env::sum_type;
use crate::infer::env::unit_type;
use crate::infra::WrapOption;
use crate::infra::WrapRc;
use crate::parser::expr::r#type::Expr;
use crate::parser::expr::test::f;

#[test]
fn test_part1() {
    let r = Expr::Match(
        None,
        Expr::EnvRef(None, "x".to_string()).wrap_rc(),
        vec![
            (
                Expr::Int(None, 1),
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "neg".to_string()).wrap_rc(),
                    Expr::Int(None, 1).wrap_rc()
                )
            ),
            (
                Expr::Int(None, 2),
                Expr::Cond(
                    None,
                    Expr::EnvRef(None, "abc".to_string()).wrap_rc(),
                    Expr::Unit(None).wrap_rc(),
                    Expr::Int(None, 0).wrap_rc()
                )
            ),
            (
                Expr::Struct(None, vec![
                    ("a".to_string(), None, Expr::Int(None, 1)),
                    ("b".to_string(), None, Expr::Discard(None)),
                    ("c".to_string(), None, Expr::Int(None, 3)),
                ]),
                Expr::Int(None, 0)
            ),
            (Expr::Discard(None), Expr::Unit(None)),
        ]
    );
    let r = Some(r);
    /*
        "match x with \
         | (1: Int -> Int -> Int) -> neg 1 \
         | 2 -> if abc then () else 0 \
         | { a = 1, b = _, c = 3 } -> 0 \
         | _ -> ()";
    */

    let seq = "match x with \
         | 1 -> neg 1 \
         | 2 -> if abc then () else 0 \
         | { a = 1, b = _, c = 3 } -> 0 \
         | _ -> ()";
    assert_eq!(f(seq), r);
    let seq = "(((\
         match x with \
         | (((1))) -> (((neg 1))) \
         | (((2))) -> (((if (((abc))) then (((()))) else (((0)))))) \
         | ((({ a = (((1))), b = (((_))), c = (((3))) }))) -> 0 \
         | (((_))) -> (((())))\
         )))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_part2() {
    let r = Expr::Match(
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
                                        vec![
                                            (Expr::Discard(None), Expr::Int(None, 114514)),
                                            (Expr::EnvRef(None, "a".to_string()), Expr::Closure(None, "x".to_string().wrap_some(), None, Expr::Closure(None, "y".to_string().wrap_some(), None, Expr::Apply(None, Expr::Apply(None, Expr::EnvRef(None, "add".to_string()).wrap_rc(), Expr::Unit(None).wrap_rc()).wrap_rc(), Expr::EnvRef(None, "y".to_string()).wrap_rc()).wrap_rc()).wrap_rc())),
                                            (Expr::EnvRef(None, "i".to_string()), Expr::Match(None, Expr::EnvRef(None, "w".to_string()).wrap_rc(), vec![(Expr::Discard(None), Expr::Int(None, 0))])),
                                        ],
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
    );
    let r = Some(r);

    let seq = "match x with \
         | 1 -> if a then b else c \
         | v -> a -> b -> add a b \
         | { a = _, b = { foo = _, bar = _ }, c = 3 } -> \
             { x = 123, y = c } \
         | _ -> \
            match y with \
            | 1 -> () \
            | () -> \
                 (a -> b -> \
                   match z with \
                   | _ -> 114514 \
                   | a -> x -> y -> add () y \
                   | i -> (match w with \
                          | _ -> 0)\
                 ) \
            | _ -> baz";

    assert_eq!(f(seq), r);

    let seq = "(((\
         match (((x))) with \
         | 1 -> if a then b else c \
         | (((v))) -> (a -> b -> (((add a b)))) \
         | { a = (((_))), b = { foo = (((_))), bar = (((_))) }, c = 3 } -> \
             ((({ x = (((123))), y = c }))) \
         | (((_))) -> \
            (((\
            match y with \
            | 1 -> () \
            | () -> \
                 (((\
                 a -> b -> \
                   (((\
                   match (((z))) with \
                   | (((_))) -> 114514 \
                   | (((a))) -> \
                       (((\
                         (((x))) -> (((y))) -> (((add () y)))\
                       )))\
                   | (((i))) -> ((((match (((w))) with \
                          | (((_))) -> (((0)))))))\
                   )))\
                 ))) \
            | _ -> baz\
            )))\
          )))";

    assert_eq!(f(seq), r);
}

#[test]
fn test_part3() {
    let r = Expr::Match(
        int_type!().wrap_some(),
        Expr::EnvRef(None, "x".to_string()).wrap_rc(),
        vec![
            (Expr::Int(int_type!().wrap_some(), 1), Expr::Cond(None, Expr::EnvRef(None, "a".to_string()).wrap_rc(), Expr::EnvRef(None, "b".to_string()).wrap_rc(), Expr::EnvRef(None, "c".to_string()).wrap_rc())),
            (Expr::EnvRef(None, "v".to_string()), Expr::Closure(None, "a".to_string().wrap_some(), None, Expr::Closure(None, "b".to_string().wrap_some(), None, Expr::Apply(None, Expr::Apply(None, Expr::EnvRef(None, "add".to_string()).wrap_rc(), Expr::EnvRef(None, "a".to_string()).wrap_rc()).wrap_rc(), Expr::EnvRef(None, "b".to_string()).wrap_rc()).wrap_rc()).wrap_rc())),
            (
                Expr::Struct(None, vec![("a".to_string(), None, Expr::Discard(None)), ("b".to_string(), None, Expr::Struct(None, vec![("foo".to_string(), None, Expr::Discard(None)), ("bar".to_string(), None, Expr::Discard(None))])), ("c".to_string(), None, Expr::Int(None, 3))]),
                Expr::Struct(sum_type![int_type!(), unit_type!()].wrap_some(), vec![("x".to_string(), None, Expr::Int(None, 123)), ("y".to_string(), None, Expr::EnvRef(None, "c".to_string()))]),
            ),
            (
                Expr::Discard(int_type!().wrap_some()),
                Expr::Match(
                    int_type!().wrap_some(),
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
                                        vec![
                                            (Expr::Discard(None), Expr::Int(None, 114514)),
                                            (Expr::EnvRef(None, "a".to_string()), Expr::Closure(None, "x".to_string().wrap_some(), None, Expr::Closure(None, "y".to_string().wrap_some(), None, Expr::Apply(None, Expr::Apply(None, Expr::EnvRef(None, "add".to_string()).wrap_rc(), Expr::Unit(None).wrap_rc()).wrap_rc(), Expr::EnvRef(None, "y".to_string()).wrap_rc()).wrap_rc()).wrap_rc())),
                                            (Expr::EnvRef(None, "i".to_string()), Expr::Match(None, Expr::EnvRef(None, "w".to_string()).wrap_rc(), vec![(Expr::Discard(None), Expr::Int(None, 0))])),
                                        ],
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
    );
    let r = Some(r);

    let seq = "(match x with \
         | (1: Int) -> if a then b else c \
         | v -> a -> b -> add a b \
         | { a = _, b = { foo = _, bar = _ }, c = 3 } -> \
             ({ x = 123, y = c }: Int | Unit) \
         | (_: Int) -> \
            (match y with \
            | 1 -> () \
            | () -> \
                 (a -> b -> \
                   match z with \
                   | _ -> 114514 \
                   | a -> x -> y -> add () y \
                   | i -> (match w with \
                          | _ -> 0)\
                 ) \
            | _ -> baz): Int\
         ): Int";

    assert_eq!(f(seq), r);
}
