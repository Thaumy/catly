use std::collections::BTreeSet;

use crate::parser::ast::parse_ast;
use crate::parser::define::Define;
use crate::parser::expr::Expr;
use crate::parser::infra::BoxExt;
use crate::parser::preprocess::blank::preprocess_blank;
use crate::parser::preprocess::comment::preprocess_comment;
use crate::parser::preprocess::keyword::preprocess_keyword;
use crate::parser::r#type::Type;

fn f(seq: &str) -> Option<Vec<Define>> {
    let seq = preprocess_comment(seq);
    let seq = preprocess_blank(&seq);
    let seq = preprocess_keyword(&seq);
    parse_ast(seq)
}

#[test]
fn test_parse_ast_part1() {
    let t1 = Define::TypeDef(
        "A".to_string(),
        Type::TypeEnvRef("B".to_string()),
    );
    let d1 = Define::ExprDef(
        "a".to_string(),
        Expr::Int(1),
    );
    let t2 = Define::TypeDef(
        "C".to_string(),
        Type::TypeEnvRef("D".to_string()),
    );
    let d2 = Define::ExprDef(
        "b".to_string(),
        Expr::Unit,
    );
    let r = vec![t1, d1, t2, d2];
    let r = Some(r);

    let seq =
        "type A = B
         def a = 1
         type C = D
         def b = ()";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_ast_part2() {
    let t1 = Define::TypeDef(
        "Foo".to_string(),
        Type::ProductType(vec![
            ("abc".to_string(), Type::TypeEnvRef("A".to_string())),
            ("uuu".to_string(), Type::TypeApply(
                Type::TypeEnvRef("List".to_string()).boxed(),
                Type::IntType.boxed(),
            )),
            ("intList".to_string(), Type::ProductType(vec![
                ("x".to_string(), Type::TypeEnvRef("X".to_string())),
                ("y".to_string(), Type::TypeEnvRef("Y".to_string())),
            ])),
        ]),
    );
    let d1 = Define::ExprDef(
        "bar".to_string(),
        Expr::Match(
            Expr::EnvRef("x".to_string()).boxed(),
            vec![
                (Expr::Int(1),
                 Expr::Cond(
                     Expr::EnvRef("a".to_string()).boxed(),
                     Expr::EnvRef("b".to_string()).boxed(),
                     Expr::EnvRef("c".to_string()).boxed(),
                 )),
                (Expr::EnvRef("v".to_string()),
                 Expr::Closure(
                     "a".to_string(),
                     Expr::Closure(
                         "b".to_string(),
                         Expr::Apply(
                             Expr::Apply(
                                 Expr::EnvRef("add".to_string()).boxed(),
                                 Expr::EnvRef("a".to_string()).boxed(),
                             ).boxed(),
                             Expr::EnvRef("b".to_string()).boxed(),
                         ).boxed(),
                     ).boxed())
                ),
                (Expr::Struct(vec![
                    ("a".to_string(), Expr::Discard),
                    ("b".to_string(),
                     Expr::Struct(vec![
                         ("foo".to_string(), Expr::Discard),
                         ("bar".to_string(), Expr::Discard),
                     ])),
                    ("c".to_string(), Expr::Int(3)),
                ]),
                 Expr::Struct(vec![
                     ("x".to_string(), Expr::Int(123)),
                     ("y".to_string(), Expr::EnvRef("c".to_string())),
                 ])),
                (Expr::Discard,
                 Expr::Match(
                     Expr::EnvRef("y".to_string()).boxed(),
                     vec![
                         (Expr::Int(1), Expr::Unit),
                         (Expr::Unit, Expr::Closure(
                             "a".to_string(),
                             Expr::Closure(
                                 "b".to_string(),
                                 Expr::Match(
                                     Expr::EnvRef("z".to_string()).boxed(),
                                     vec![
                                         (Expr::Discard, Expr::Int(114514)),
                                         (Expr::EnvRef("a".to_string()),
                                          Expr::Closure(
                                              "x".to_string(),
                                              Expr::Closure(
                                                  "y".to_string(),
                                                  Expr::Apply(
                                                      Expr::Apply(
                                                          Expr::EnvRef("add".to_string()).boxed(),
                                                          Expr::Unit.boxed(),
                                                      ).boxed(),
                                                      Expr::EnvRef("y".to_string()).boxed(),
                                                  ).boxed(),
                                              ).boxed(),
                                          )),
                                     ],
                                 ).boxed(),
                             ).boxed(),
                         )),
                         (Expr::Discard, Expr::EnvRef("baz".to_string())),
                     ],
                 )),
            ],
        ),
    );
    let t2 = Define::TypeDef(
        "Love".to_string(),
        Type::SumType(BTreeSet::from([
            Type::TypeEnvRef("A".to_string()),
            Type::UnitType,
            Type::TypeEnvRef("C".to_string()),
            Type::IntType,
        ])),
    );
    let d2 = Define::ExprDef(
        "main".to_string(),
        Expr::Let(
            "a".to_string(),
            Expr::Int(123).boxed(),
            Expr::Let(
                "b".to_string(),
                Expr::Let("x".to_string(),
                          Expr::Closure(
                              "i".to_string(),
                              Expr::Closure(
                                  "j".to_string(),
                                  Expr::EnvRef("k".to_string()).boxed(),
                              ).boxed(),
                          ).boxed(),
                          Expr::Let(
                              "y".to_string(),
                              Expr::EnvRef("a".to_string()).boxed(),
                              Expr::Let(
                                  "z".to_string(),
                                  Expr::Unit.boxed(),
                                  Expr::EnvRef("a".to_string()).boxed(),
                              ).boxed(),
                          ).boxed(),
                ).boxed(),
                Expr::Let(
                    "d".to_string(),
                    Expr::Apply(
                        Expr::EnvRef("neg".to_string()).boxed(),
                        Expr::Int(1).boxed(),
                    ).boxed(),
                    Expr::Let(
                        "e".to_string(),
                        Expr::Int(6).boxed(),
                        Expr::Let(
                            "k".to_string(),
                            Expr::Unit.boxed(),
                            Expr::Let(
                                "m".to_string(),
                                Expr::Unit.boxed(),
                                Expr::Let(
                                    "n".to_string(),
                                    Expr::Int(4).boxed(),
                                    Expr::Apply(
                                        Expr::Apply(
                                            Expr::EnvRef("add".to_string()).boxed(),
                                            Expr::Unit.boxed(),
                                        ).boxed(),
                                        Expr::Int(456).boxed(),
                                    ).boxed(),
                                ).boxed(),
                            ).boxed(),
                        ).boxed(),
                    ).boxed(),
                ).boxed(),
            ).boxed(),
        ),
    );
    let r = vec![t1, d1, t2, d2];
    let r = Some(r);

    let seq =
        "type Foo = { abc: A, uuu: List Int, intList: { x: X, y: Y } }
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
         def main =
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
