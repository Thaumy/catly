use crate::parser::expr::{Expr, parse_expr};
use crate::parser::infra::{BoxExt, MaybeExpr};
use crate::parser::preprocess::blank::preprocess_blank;
use crate::parser::preprocess::comment::preprocess_comment;
use crate::parser::preprocess::keyword::preprocess_keyword;

fn f(seq: &str) -> MaybeExpr {
    let seq = preprocess_comment(seq);
    let seq = preprocess_blank(&seq);
    let seq = preprocess_keyword(&seq);
    parse_expr(seq)
}

#[test]
fn test_parse_unit() {
    let r = Expr::Unit(None);
    let r = Some(r);

    assert_eq!(f("()"), r);
    assert_eq!(f("(())"), r);
    assert_eq!(f("((()))"), r);
}

#[test]
fn test_parse_int() {
    let r = Expr::Int(None, 123);
    let r = Some(r);

    assert_eq!(f("123"), r);
    assert_eq!(f("(123)"), r);
    assert_eq!(f("((123))"), r);
}

#[test]
fn test_parse_env_ref() {
    let r = Expr::EnvRef("abc".to_string());
    let r = Some(r);

    assert_eq!(f("abc"), r);
    assert_eq!(f("(abc)"), r);
    assert_eq!(f("((abc))"), r);
}

#[test]
fn test_parse_apply_part1() {
    // Apply(Unit, Int)
    let r = Expr::Apply(
        None,
        Expr::Unit(None).boxed(),
        Expr::Int(None, 123).boxed(),
    );
    let r = Some(r);

    assert_eq!(f("() 123"), r);
    assert_eq!(f("(()) (123)"), r);
    assert_eq!(f("((())) ((123))"), r);
    assert_eq!(f("(((())) ((123)))"), r);
    assert_eq!(f("((((())) ((123))))"), r);
}

#[test]
fn test_parse_apply_part2() {
    // Apply(EnvRef, Int)
    let r = Expr::Apply(
        None,
        Expr::EnvRef("abc".to_string()).boxed(),
        Expr::Int(None, 123).boxed(),
    );
    let r = Some(r);

    assert_eq!(f("abc 123"), r);
    assert_eq!(f("(abc) (123)"), r);
    assert_eq!(f("((abc)) ((123))"), r);
    assert_eq!(f("(((abc)) ((123)))"), r);
    assert_eq!(f("((((abc)) ((123))))"), r);
}

#[test]
fn test_parse_apply_part3() {
    // Apply(EnvRef, Unit)
    let r = Expr::Apply(
        None,
        Expr::EnvRef("abc".to_string()).boxed(),
        Expr::Unit(None).boxed(),
    );
    let r = Some(r);

    assert_eq!(f("abc ()"), r);
    assert_eq!(f("(abc) (())"), r);
    assert_eq!(f("((abc)) ((()))"), r);
    assert_eq!(f("(((abc)) ((())))"), r);
    assert_eq!(f("((((abc)) ((()))))"), r);
}

#[test]
fn test_parse_apply_part4() {
    // Apply(EnvRef, Apply(EnvRef, Unit))
    let r = Expr::Apply(
        None,
        Expr::EnvRef("abc".to_string()).boxed(),
        Expr::Apply(
            None,
            Expr::EnvRef("abc".to_string()).boxed(),
            Expr::Unit(None).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    assert_eq!(f("abc (abc ())"), r);
    assert_eq!(f("(abc) ((abc ()))"), r);
    assert_eq!(f("((abc)) (((abc ())))"), r);
    assert_eq!(f("(((abc)) (((abc ()))))"), r);
    assert_eq!(f("((((abc)) (((abc ())))))"), r);
}

#[test]
fn test_parse_apply_part5() {
    // Apply(EnvRef, Apply(EnvRef, Apply(EnvRef, Unit)))
    let r = Expr::Apply(
        None,
        Expr::EnvRef("abc".to_string()).boxed(),
        Expr::Apply(
            None,
            Expr::EnvRef("abc".to_string()).boxed(),
            Expr::Apply(
                None,
                Expr::EnvRef("abc".to_string()).boxed(),
                Expr::Unit(None).boxed(),
            ).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    assert_eq!(f("abc (abc (abc ()))"), r);
    assert_eq!(f("(abc) ((abc (abc ())))"), r);
    assert_eq!(f("((abc)) (((abc (abc ()))))"), r);
    assert_eq!(f("(((abc)) (((abc (abc ())))))"), r);
    assert_eq!(f("((((abc)) (((abc (abc ()))))))"), r);
}

#[test]
fn test_parse_apply_part6() {
    // Apply(EnvRef, Apply(EnvRef, Unit))
    let r = Expr::Apply(
        None,
        Expr::Apply(
            None,
            Expr::EnvRef("abc".to_string()).boxed(),
            Expr::Int(None, 123).boxed(),
        ).boxed(),
        Expr::Apply(
            None,
            Expr::Apply(
                None,
                Expr::EnvRef("add".to_string()).boxed(),
                Expr::Int(None, 123).boxed(),
            ).boxed(),
            Expr::Int(None, 456).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    assert_eq!(f("abc 123 (add 123 456)"), r);
    assert_eq!(f("abc ((123)) (((add 123 456)))"), r);
    assert_eq!(f("(((abc (((123))) (((add (((123))) (((456)))))))))"), r);
}

#[test]
fn test_parse_apply_part7() {
    // Apply(EnvRef, Apply(EnvRef, Unit))
    let r = Expr::Apply(
        None,
        Expr::Apply(
            None,
            Expr::EnvRef("abc".to_string()).boxed(),
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef("add".to_string()).boxed(),
                    Expr::Int(None, 123).boxed(),
                ).boxed(),
                Expr::Int(None, 456).boxed(),
            ).boxed(),
        ).boxed(),
        Expr::Int(None, 123).boxed(),
    );
    let r = Some(r);

    assert_eq!(f("abc (add 123 456) 123"), r);
    assert_eq!(f("abc (((add 123 456))) ((123))"), r);
    assert_eq!(f("(((abc (((add (((123))) (((456)))))) (((123))))))"), r);
}

#[test]
fn test_parse_cond_part1() {
    // Cond(EnvRef, Int, Unit)
    let r = Expr::Cond(
        None,
        Expr::EnvRef("abc".to_string()).boxed(),
        Expr::Int(None, 123).boxed(),
        Expr::Unit(None).boxed(),
    );
    let r = Some(r);

    assert_eq!(f("if abc then 123 else ()"), r);
    assert_eq!(f("if ((abc)) then ((123)) else ((()))"), r);
    assert_eq!(f("(if (((abc))) then (((123))) else (((()))))"), r);
    assert_eq!(f("(((if (((abc))) then (((123))) else (((()))))))"), r);
}

#[test]
fn test_parse_cond_part2() {
    // Cond(a, a, a)
    // while: a = Cond(EnvRef, Apply(Int, Unit), Int)
    let e = Expr::Cond(
        None,
        Expr::EnvRef("abc".to_string()).boxed(),
        Expr::Apply(
            None,
            Expr::Int(None, 123).boxed(),
            Expr::Unit(None).boxed(),
        ).boxed(),
        Expr::Int(None, 456).boxed(),
    );
    let r = Some(Expr::Cond(
        None,
        e.clone().boxed(),
        e.clone().boxed(),
        e.clone().boxed(),
    ));

    let e = "if abc then 123 () else 456";
    let seq = &format!("if {} then {} else {}", e, e, e);
    assert_eq!(f(seq), r);
    let e = "if abc then (123 ()) else 456";
    let seq = &format!("if {} then {} else {}", e, e, e);
    assert_eq!(f(seq), r);
    let e = "(((if ((abc)) then ((123 ())) else ((456)))))";
    let seq = &format!("if {} then {} else {}", e, e, e);
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_cond_part3() {
    // Cond(b, b, b)
    // while: a = Cond(Apply(Int, Unit), Int, EnvRef)
    // while: b = Cond(a, a, a)
    let a = Expr::Cond(
        None,
        Expr::Apply(
            None,
            Expr::Int(None, 123).boxed(),
            Expr::Unit(None).boxed(),
        ).boxed(),
        Expr::Int(None, 123).boxed(),
        Expr::EnvRef("abc".to_string()).boxed(),
    );
    let b = Expr::Cond(
        None,
        a.clone().boxed(),
        a.clone().boxed(),
        a.clone().boxed(),
    );
    let r = Expr::Cond(
        None,
        b.clone().boxed(),
        b.clone().boxed(),
        b.clone().boxed(),
    );
    let r = Some(r);

    let a = "if 123 () then 123 else abc";
    let b = &format!("if {} then {} else {}", a, a, a);
    let seq = &format!("if {} then {} else {}", b, b, b);
    assert_eq!(f(seq), r);
    let a = "(((if (((123 ()))) then (((123))) else (((abc))))))";
    let b = &format!("(((if {} then {} else {})))", a, a, a);
    let seq = &format!("if {} then {} else {}", b, b, b);
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_cond_part4() {
    // Cond(b, b, b)
    // while: a = Cond(Apply(Int, Unit), Int, EnvRef)
    // while: b = Cond(a, a, a)
    let a = Expr::Cond(
        None,
        Expr::Apply(
            None,
            Expr::Int(None, 123).boxed(),
            Expr::Unit(None).boxed(),
        ).boxed(),
        Expr::Int(None, 123).boxed(),
        Expr::EnvRef("abc".to_string()).boxed(),
    );
    let b = Expr::Cond(
        None,
        a.clone().boxed(),
        a.clone().boxed(),
        a.clone().boxed(),
    );
    let r = Expr::Cond(
        None,
        b.clone().boxed(),
        b.clone().boxed(),
        b.clone().boxed(),
    );
    let r = Some(r);

    let a = "(((if (((123 ()))) then (((123))) else (((abc))))))";
    let b = &format!("(((if ((({}))) then ((({}))) else {})))", a, a, a);
    let seq = &format!("(((if ((({}))) then {} else ((({}))))))", b, b, b);
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_closure_part1() {
    let r = Expr::Closure(
        None,
        "a".to_string(),
        None,
        Expr::Apply(
            None,
            Expr::Apply(
                None,
                Expr::EnvRef("add".to_string()).boxed(),
                Expr::Int(None, 123).boxed(),
            ).boxed(),
            Expr::Unit(None).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    let seq = "a -> add 123 ()";
    assert_eq!(f(seq), r);
    let seq = "(a -> (add (123) (())))";
    assert_eq!(f(seq), r);
    let seq = "(((a -> ((((add 123)) ((())))))))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_closure_part2() {
    let r = Expr::Closure(
        None,
        "a".to_string(),
        None,
        Expr::Closure(
            None,
            "b".to_string(),
            None,
            Expr::Closure(
                None,
                "c".to_string(),
                None,
                Expr::Apply(
                    None,
                    Expr::Apply(
                        None,
                        Expr::EnvRef("add".to_string()).boxed(),
                        Expr::Apply(
                            None,
                            Expr::Apply(
                                None,
                                Expr::EnvRef("add".to_string()).boxed(),
                                Expr::EnvRef("a".to_string()).boxed(),
                            ).boxed(),
                            Expr::EnvRef("b".to_string()).boxed(),
                        ).boxed(),
                    ).boxed(),
                    Expr::EnvRef("c".to_string()).boxed(),
                ).boxed(),
            ).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    let seq = "a -> b -> c -> add (add a b) c";
    assert_eq!(f(seq), r);
    let seq = "((a -> ((b -> ((c -> ((add (((add (a) (b)))) (c)))))))))";
    assert_eq!(f(seq), r);
    let seq = "((((((a))) -> (((b -> (((c))) -> (((add))) (add a b) c))))))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_closure_part3() {
    let r = Expr::Closure(
        None,
        "aaa".to_string(),
        None,
        Expr::Closure(
            None,
            "bbb".to_string(),
            None,
            Expr::Closure(
                None,
                "ccc".to_string(),
                None,
                Expr::Apply(
                    None,
                    Expr::Apply(
                        None,
                        Expr::EnvRef("add".to_string()).boxed(),
                        Expr::Apply(
                            None,
                            Expr::Apply(
                                None,
                                Expr::EnvRef("add".to_string()).boxed(),
                                Expr::EnvRef("aaa".to_string()).boxed(),
                            ).boxed(),
                            Expr::Int(None, 123).boxed(),
                        ).boxed(),
                    ).boxed(),
                    Expr::EnvRef("ccc".to_string()).boxed(),
                ).boxed(),
            ).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    let seq = "aaa -> bbb -> ccc -> add (add aaa 123) ccc";
    assert_eq!(f(seq), r);
    let seq = "(((aaa -> ((bbb -> (ccc -> ((((((add (add aaa 123)))) ccc)))))))))";
    assert_eq!(f(seq), r);
    let seq = "(((aaa -> (((((bbb))) -> (((ccc)) -> ((((((add (add (((aaa))) 123)))) ccc)))))))))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_struct_part1() {
    let r = Expr::Struct(
        None,
        vec![
            ("a".to_string(), None, Expr::Int(None, 123)),
            ("ab".to_string(), None, Expr::EnvRef("ref".to_string())),
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
fn test_parse_struct_part2() {
    let a = Expr::Struct(
        None,
        vec![
            ("abc".to_string(),
             None,
             Expr::Struct(
                 None,
                 vec![
                     ("efg".to_string(),
                      None,
                      Expr::Cond(
                          None,
                          Expr::Int(None, 123).boxed(),
                          Expr::Unit(None).boxed(),
                          Expr::Int(None, 0).boxed(),
                      ))
                 ])),
            ("x".to_string(), None, Expr::Int(None, 1)),
        ]);
    let fun = Expr::Closure(
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
                    Expr::EnvRef("x".to_string()).boxed(),
                ).boxed(),
                Expr::EnvRef("y".to_string()).boxed(),
            ).boxed(),
        ).boxed(),
    );
    let r = Expr::Struct(
        None,
        vec![
            ("a".to_string(), None, a),
            ("ab".to_string(), None, Expr::Apply(
                None,
                Expr::EnvRef("neg".to_string()).boxed(),
                Expr::Int(None, 1).boxed(),
            )),
            ("fun".to_string(), None, fun),
        ]);
    let r = Some(r);

    let seq =
        "{ \
               a = { abc = { efg = if 123 then () else 0 }, x = 1 }, \
               ab = neg 1, \
               fun = (x -> y -> add x y) \
             }";
    assert_eq!(f(seq), r);
    let seq =
        "((({ \
                  a = ((({ abc = { efg = if 123 then ((())) else 0 }, x = 1 }))), \
                  ab = (((neg))) 1, \
                  fun = (x -> y -> add x y) \
            })))";
    assert_eq!(f(seq), r);
    let seq =
        "((({ \
                  (((a))) = ((({ abc = { efg = if (((123))) then ((())) else 0 }, x = (((1))) }))), \
                  (((ab))) = ((((((neg))) (((1)))))), \
                  (((fun))) = (x -> (((y -> add x y)))) \
            })))";
    assert_eq!(f(seq), r);
}

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
         | v -> (a -> b -> add a b) \
         | { a = _, b = { foo = _, bar = _ }, c = 3 } -> \
             { x = 123, y = c } \
         | _ -> \
            match y with \
            | 1 -> () \
            | () -> \
                 (a -> b -> \
                   match z with \
                   | _ -> 114514 \
                   | a -> (x -> y -> add () y)\
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
                   )))\
                 ))) \
            | _ -> baz\
            )))\
          )))";

    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_let_part1() {
    let r = Expr::Let(
        None,
        "a".to_string(),
        None,
        Expr::Int(None, 123).boxed(),
        Expr::Apply(
            None,
            Expr::Apply(
                None,
                Expr::EnvRef("add".to_string()).boxed(),
                Expr::EnvRef("a".to_string()).boxed(),
            ).boxed(),
            Expr::Int(None, 456).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    let seq = "let a = 123 in add a 456";
    assert_eq!(f(seq), r);
    let seq = "let a = 123,in add a 456";
    assert_eq!(f(seq), r);
    let seq = "(((let (((a))) = (((123))) in (((add a (((456)))))))))";
    assert_eq!(f(seq), r);
    let seq = "(((let (((a))) = (((123))),in (((add a (((456)))))))))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_let_part2() {
    let r = Expr::Let(
        None,
        "a".to_string(),
        None,
        Expr::Int(None, 123).boxed(),
        Expr::Let(
            None,
            "b".to_string(),
            None,
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef("add".to_string()).boxed(),
                    Expr::EnvRef("c".to_string()).boxed(),
                ).boxed(),
                Expr::EnvRef("d".to_string()).boxed(),
            ).boxed(),
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef("add".to_string()).boxed(),
                    Expr::Unit(None).boxed(),
                ).boxed(),
                Expr::Int(None, 456).boxed(),
            ).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    let seq = "let a = 123, b = add c d in add () 456";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_let_part3() {
    let r = Expr::Let(
        None,
        "a".to_string(),
        None,
        Expr::Int(None, 123).boxed(),
        Expr::Let(
            None,
            "b".to_string(),
            None,
            Expr::Let(
                None,
                "x".to_string(),
                None,
                Expr::Closure(
                    None,
                    "i".to_string(),
                    None,
                    Expr::Closure(
                        None,
                        "j".to_string(),
                        None,
                        Expr::EnvRef("k".to_string()).boxed(),
                    ).boxed(),
                ).boxed(),
                Expr::Let(
                    None,
                    "y".to_string(),
                    None,
                    Expr::EnvRef("a".to_string()).boxed(),
                    Expr::Let(
                        None,
                        "z".to_string(),
                        None,
                        Expr::Unit(None).boxed(),
                        Expr::EnvRef("a".to_string()).boxed(),
                    ).boxed(),
                ).boxed(),
            ).boxed(),
            Expr::Let(
                None,
                "d".to_string(),
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef("neg".to_string()).boxed(),
                    Expr::Int(None,1).boxed(),
                ).boxed(),
                Expr::Let(
                    None,
                    "e".to_string(),
                    None,
                    Expr::Int(None, 6).boxed(),
                    Expr::Let(
                        None,
                        "k".to_string(),
                        None,
                        Expr::Unit(None).boxed(),
                        Expr::Let(
                            None,
                            "m".to_string(),
                            None,
                            Expr::Unit(None).boxed(),
                            Expr::Let(
                                None,
                                "n".to_string(),
                                None,
                                Expr::Int(None, 4).boxed(),
                                Expr::Apply(
                                    None,
                                    Expr::Apply(
                                        None,
                                        Expr::EnvRef("add".to_string()).boxed(),
                                        Expr::Unit(None).boxed(),
                                    ).boxed(),
                                    Expr::Int(None, 456).boxed(),
                                ).boxed(),
                            ).boxed(),
                        ).boxed(),
                    ).boxed(),
                ).boxed(),
            ).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    let seq =
        "let a = 123, \
                 b = \
                 let x = i -> j -> k, \
                     y = a \
                 in let z = () in a, \
                 d = neg 1 \
             in \
             let e = 6, k = () in \
             let m = (), n = 4 in \
             add () 456";
    assert_eq!(f(seq), r);
    let seq =
        "let a = (((123))), \
                 b = \
                 (((\
                     let x = ((((((i))) -> ((((((j))) -> (((k))))))))), \
                         y = (((a))) \
                     in (((\
                        let (((z))) = (((()))) in (((a)))\
                        )))\
                 ))), \
                 (((d))) = \
                     (((\
                         (((neg))) (((1)))\
                     ))) \
             in \
             (((\
             let (((e))) = (((6))), (((k))) = (((()))) in \
                 (((\
                 let (((m))) = (((()))), (((n))) = (((4))) in \
                     (((\
                     add () (((456)))\
                     )))\
                 )))\
             )))";
    assert_eq!(f(seq), r);
}
