use crate::btree_set;
use crate::infra::option::AnyExt;
use crate::infra::r#box::Ext;
use crate::parser::expr::test::f;
use crate::parser::expr::Expr;
use crate::parser::r#type::Type;

#[test]
fn test_parse_match_part1() {
    let r = Expr::Match(
        None,
        Expr::EnvRef(None, "x".to_string()).boxed(),
        vec![
            (
                Expr::Int(None, 1),
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "neg".to_string()).boxed(),
                    Expr::Int(None, 1).boxed()
                )
            ),
            (
                Expr::Int(None, 2),
                Expr::Cond(
                    None,
                    Expr::EnvRef(None, "abc".to_string()).boxed(),
                    Expr::Unit(None).boxed(),
                    Expr::Int(None, 0).boxed()
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
fn test_parse_match_part2() {
    let r = Expr::Match(
        None,
        Expr::EnvRef(None, "x".to_string()).boxed(),
        vec![
            (Expr::Int(None, 1), Expr::Cond(None, Expr::EnvRef(None, "a".to_string()).boxed(), Expr::EnvRef(None, "b".to_string()).boxed(), Expr::EnvRef(None, "c".to_string()).boxed())),
            (Expr::EnvRef(None, "v".to_string()), Expr::Closure(None, "a".to_string().some(), None, Expr::Closure(None, "b".to_string().some(), None, Expr::Apply(None, Expr::Apply(None, Expr::EnvRef(None, "add".to_string()).boxed(), Expr::EnvRef(None, "a".to_string()).boxed()).boxed(), Expr::EnvRef(None, "b".to_string()).boxed()).boxed()).boxed())),
            (Expr::Struct(None, vec![("a".to_string(), None, Expr::Discard(None)), ("b".to_string(), None, Expr::Struct(None, vec![("foo".to_string(), None, Expr::Discard(None)), ("bar".to_string(), None, Expr::Discard(None))])), ("c".to_string(), None, Expr::Int(None, 3))]), Expr::Struct(None, vec![("x".to_string(), None, Expr::Int(None, 123)), ("y".to_string(), None, Expr::EnvRef(None, "c".to_string()))])),
            (
                Expr::Discard(None),
                Expr::Match(
                    None,
                    Expr::EnvRef(None, "y".to_string()).boxed(),
                    vec![
                        (Expr::Int(None, 1), Expr::Unit(None)),
                        (
                            Expr::Unit(None),
                            Expr::Closure(
                                None,
                                "a".to_string().some(),
                                None,
                                Expr::Closure(
                                    None,
                                    "b".to_string().some(),
                                    None,
                                    Expr::Match(
                                        None,
                                        Expr::EnvRef(None, "z".to_string()).boxed(),
                                        vec![
                                            (Expr::Discard(None), Expr::Int(None, 114514)),
                                            (Expr::EnvRef(None, "a".to_string()), Expr::Closure(None, "x".to_string().some(), None, Expr::Closure(None, "y".to_string().some(), None, Expr::Apply(None, Expr::Apply(None, Expr::EnvRef(None, "add".to_string()).boxed(), Expr::Unit(None).boxed()).boxed(), Expr::EnvRef(None, "y".to_string()).boxed()).boxed()).boxed())),
                                            (Expr::EnvRef(None, "i".to_string()), Expr::Match(None, Expr::EnvRef(None, "w".to_string()).boxed(), vec![(Expr::Discard(None), Expr::Int(None, 0))])),
                                        ],
                                    )
                                    .boxed(),
                                )
                                .boxed(),
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
fn test_parse_match_part3() {
    let r = Expr::Match(
        Type::TypeEnvRef("Int".to_string()).some(),
        Expr::EnvRef(None, "x".to_string()).boxed(),
        vec![
            (Expr::Int(Type::TypeEnvRef("Int".to_string()).some(), 1), Expr::Cond(None, Expr::EnvRef(None, "a".to_string()).boxed(), Expr::EnvRef(None, "b".to_string()).boxed(), Expr::EnvRef(None, "c".to_string()).boxed())),
            (Expr::EnvRef(None, "v".to_string()), Expr::Closure(None, "a".to_string().some(), None, Expr::Closure(None, "b".to_string().some(), None, Expr::Apply(None, Expr::Apply(None, Expr::EnvRef(None, "add".to_string()).boxed(), Expr::EnvRef(None, "a".to_string()).boxed()).boxed(), Expr::EnvRef(None, "b".to_string()).boxed()).boxed()).boxed())),
            (
                Expr::Struct(None, vec![("a".to_string(), None, Expr::Discard(None)), ("b".to_string(), None, Expr::Struct(None, vec![("foo".to_string(), None, Expr::Discard(None)), ("bar".to_string(), None, Expr::Discard(None))])), ("c".to_string(), None, Expr::Int(None, 3))]),
                Expr::Struct(Type::SumType(btree_set![Type::TypeEnvRef("Int".to_string()), Type::TypeEnvRef("Unit".to_string()),]).some(), vec![("x".to_string(), None, Expr::Int(None, 123)), ("y".to_string(), None, Expr::EnvRef(None, "c".to_string()))]),
            ),
            (
                Expr::Discard(Type::TypeEnvRef("Int".to_string()).some()),
                Expr::Match(
                    Type::TypeEnvRef("Int".to_string()).some(),
                    Expr::EnvRef(None, "y".to_string()).boxed(),
                    vec![
                        (Expr::Int(None, 1), Expr::Unit(None)),
                        (
                            Expr::Unit(None),
                            Expr::Closure(
                                None,
                                "a".to_string().some(),
                                None,
                                Expr::Closure(
                                    None,
                                    "b".to_string().some(),
                                    None,
                                    Expr::Match(
                                        None,
                                        Expr::EnvRef(None, "z".to_string()).boxed(),
                                        vec![
                                            (Expr::Discard(None), Expr::Int(None, 114514)),
                                            (Expr::EnvRef(None, "a".to_string()), Expr::Closure(None, "x".to_string().some(), None, Expr::Closure(None, "y".to_string().some(), None, Expr::Apply(None, Expr::Apply(None, Expr::EnvRef(None, "add".to_string()).boxed(), Expr::Unit(None).boxed()).boxed(), Expr::EnvRef(None, "y".to_string()).boxed()).boxed()).boxed())),
                                            (Expr::EnvRef(None, "i".to_string()), Expr::Match(None, Expr::EnvRef(None, "w".to_string()).boxed(), vec![(Expr::Discard(None), Expr::Int(None, 0))])),
                                        ],
                                    )
                                    .boxed(),
                                )
                                .boxed(),
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