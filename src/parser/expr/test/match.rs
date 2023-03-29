use crate::parser::expr::Expr;
use crate::parser::expr::test::f;
use crate::parser::infra::r#box::Ext;

#[test]
fn test_parse_match_part1() {
    let r = Expr::Match(
        None,
        Expr::EnvRef("x".to_string()).boxed(),
        vec![
            (Expr::Int(None, 1),
             Expr::Apply(
                 None,
                 Expr::EnvRef("neg".to_string()).boxed(),
                 Expr::Int(None, 1).boxed(),
             )),
            (Expr::Int(None, 2),
             Expr::Cond(
                 None,
                 Expr::EnvRef("abc".to_string()).boxed(),
                 Expr::Unit(None).boxed(),
                 Expr::Int(None, 0).boxed(),
             )),
            (Expr::Struct(
                None,
                vec![
                    ("a".to_string(), None, Expr::Int(None, 1)),
                    ("b".to_string(), None, Expr::Discard),
                    ("c".to_string(), None, Expr::Int(None, 3)),
                ]),
             Expr::Int(None, 0)),
            (Expr::Discard,
             Expr::Unit(None)),
        ],
    );
    let r = Some(r);
    /*
        "match x with \
         | (1: Int -> Int -> Int) -> neg 1 \
         | 2 -> if abc then () else 0 \
         | { a = 1, b = _, c = 3 } -> 0 \
         | _ -> ()";
    */

    let seq =
        "match x with \
         | 1 -> neg 1 \
         | 2 -> if abc then () else 0 \
         | { a = 1, b = _, c = 3 } -> 0 \
         | _ -> ()";
    assert_eq!(f(seq), r);
    let seq =
        "(((\
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
        Expr::EnvRef("x".to_string()).boxed(),
        vec![
            (Expr::Int(None, 1),
             Expr::Cond(
                 None,
                 Expr::EnvRef("a".to_string()).boxed(),
                 Expr::EnvRef("b".to_string()).boxed(),
                 Expr::EnvRef("c".to_string()).boxed(),
             )),
            (Expr::EnvRef("v".to_string()),
             Expr::Closure(
                 None,
                 "a".to_string(),
                 None,
                 Expr::Closure(
                     None,
                     "b".to_string(),
                     None,
                     Expr::Apply(
                         None,
                         Expr::Apply(
                             None,
                             Expr::EnvRef("add".to_string()).boxed(),
                             Expr::EnvRef("a".to_string()).boxed(),
                         ).boxed(),
                         Expr::EnvRef("b".to_string()).boxed(),
                     ).boxed(),
                 ).boxed())
            ),
            (Expr::Struct(
                None,
                vec![
                    ("a".to_string(),
                     None,
                     Expr::Discard),
                    ("b".to_string(),
                     None,
                     Expr::Struct(
                         None,
                         vec![
                             ("foo".to_string(), None, Expr::Discard),
                             ("bar".to_string(), None, Expr::Discard),
                         ])),
                    ("c".to_string(),
                     None,
                     Expr::Int(None, 3)),
                ]),
             Expr::Struct(
                 None,
                 vec![
                     ("x".to_string(),
                      None,
                      Expr::Int(None, 123)),
                     ("y".to_string(),
                      None,
                      Expr::EnvRef("c".to_string())),
                 ])),
            (Expr::Discard,
             Expr::Match(
                 None,
                 Expr::EnvRef("y".to_string()).boxed(),
                 vec![
                     (Expr::Int(None, 1), Expr::Unit(None)),
                     (Expr::Unit(None), Expr::Closure(
                         None,
                         "a".to_string(),
                         None,
                         Expr::Closure(
                             None,
                             "b".to_string(),
                             None,
                             Expr::Match(
                                 None,
                                 Expr::EnvRef("z".to_string()).boxed(),
                                 vec![
                                     (Expr::Discard, Expr::Int(None, 114514)),
                                     (Expr::EnvRef("a".to_string()),
                                      Expr::Closure(
                                          None,
                                          "x".to_string(),
                                          None,
                                          Expr::Closure(
                                              None,
                                              "y".to_string(),
                                              None,
                                              Expr::Apply(
                                                  None,
                                                  Expr::Apply(
                                                      None,
                                                      Expr::EnvRef("add".to_string()).boxed(),
                                                      Expr::Unit(None).boxed(),
                                                  ).boxed(),
                                                  Expr::EnvRef("y".to_string()).boxed(),
                                              ).boxed(),
                                          ).boxed(),
                                      )),
                                     (Expr::EnvRef("i".to_string()),
                                      Expr::Match(
                                          None,
                                          Expr::EnvRef("w".to_string()).boxed(),
                                          vec![
                                              (Expr::Discard, Expr::Int(None, 0)),
                                          ],
                                      )),
                                 ],
                             ).boxed(),
                         ).boxed(),
                     )),
                     (Expr::Discard, Expr::EnvRef("baz".to_string())),
                 ],
             )),
        ],
    );
    let r = Some(r);

    let seq =
        "match x with \
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

    let seq =
        "(((\
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
