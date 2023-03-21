use crate::parser::BoxExt;
use crate::parser::expr::{Expr, parse_expr};

#[test]
fn test_parse_expr_unit() {
    let r = Expr::Unit;
    let r = Some(r);

    assert_eq!(parse_expr("()"), r);
    assert_eq!(parse_expr("(())"), r);
    assert_eq!(parse_expr("((()))"), r);
}

#[test]
fn test_parse_expr_int() {
    let r = Expr::Int(123);
    let r = Some(r);

    assert_eq!(parse_expr("123"), r);
    assert_eq!(parse_expr("(123)"), r);
    assert_eq!(parse_expr("((123))"), r);
}

#[test]
fn test_parse_expr_env_ref() {
    let r = Expr::EnvRef("abc".to_string());
    let r = Some(r);

    assert_eq!(parse_expr("abc"), r);
    assert_eq!(parse_expr("(abc)"), r);
    assert_eq!(parse_expr("((abc))"), r);
}

#[test]
fn test_parse_expr_apply_part1() {
    // Apply(Unit, Int)
    let r = Expr::Apply(
        Expr::Unit.boxed(),
        Expr::Int(123).boxed(),
    );
    let r = Some(r);

    assert_eq!(parse_expr("() 123"), r);
    assert_eq!(parse_expr("(()) (123)"), r);
    assert_eq!(parse_expr("((())) ((123))"), r);
    assert_eq!(parse_expr("(((())) ((123)))"), r);
    assert_eq!(parse_expr("((((())) ((123))))"), r);
}

#[test]
fn test_parse_expr_apply_part2() {
    // Apply(EnvRef, Int)
    let r = Expr::Apply(
        Expr::EnvRef("abc".to_string()).boxed(),
        Expr::Int(123).boxed(),
    );
    let r = Some(r);

    assert_eq!(parse_expr("abc 123"), r);
    assert_eq!(parse_expr("(abc) (123)"), r);
    assert_eq!(parse_expr("((abc)) ((123))"), r);
    assert_eq!(parse_expr("(((abc)) ((123)))"), r);
    assert_eq!(parse_expr("((((abc)) ((123))))"), r);
}

#[test]
fn test_parse_expr_apply_part3() {
    // Apply(EnvRef, Unit)
    let r = Expr::Apply(
        Expr::EnvRef("abc".to_string()).boxed(),
        Expr::Unit.boxed(),
    );
    let r = Some(r);

    assert_eq!(parse_expr("abc ()"), r);
    assert_eq!(parse_expr("(abc) (())"), r);
    assert_eq!(parse_expr("((abc)) ((()))"), r);
    assert_eq!(parse_expr("(((abc)) ((())))"), r);
    assert_eq!(parse_expr("((((abc)) ((()))))"), r);
}

#[test]
fn test_parse_expr_apply_part4() {
    // Apply(EnvRef, Apply(EnvRef, Unit))
    let r = Expr::Apply(
        Expr::EnvRef("abc".to_string()).boxed(),
        Expr::Apply(
            Expr::EnvRef("abc".to_string()).boxed(),
            Expr::Unit.boxed(),
        ).boxed(),
    );
    let r = Some(r);

    assert_eq!(parse_expr("abc (abc ())"), r);
    assert_eq!(parse_expr("(abc) ((abc ()))"), r);
    assert_eq!(parse_expr("((abc)) (((abc ())))"), r);
    assert_eq!(parse_expr("(((abc)) (((abc ()))))"), r);
    assert_eq!(parse_expr("((((abc)) (((abc ())))))"), r);
}

#[test]
fn test_parse_expr_apply_part5() {
    // Apply(EnvRef, Apply(EnvRef, Apply(EnvRef, Unit)))
    let r = Expr::Apply(
        Expr::EnvRef("abc".to_string()).boxed(),
        Expr::Apply(
            Expr::EnvRef("abc".to_string()).boxed(),
            Expr::Apply(
                Expr::EnvRef("abc".to_string()).boxed(),
                Expr::Unit.boxed(),
            ).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    assert_eq!(parse_expr("abc (abc (abc ()))"), r);
    assert_eq!(parse_expr("(abc) ((abc (abc ())))"), r);
    assert_eq!(parse_expr("((abc)) (((abc (abc ()))))"), r);
    assert_eq!(parse_expr("(((abc)) (((abc (abc ())))))"), r);
    assert_eq!(parse_expr("((((abc)) (((abc (abc ()))))))"), r);
}

#[test]
fn test_parse_expr_apply_part6() {
    // Apply(EnvRef, Apply(EnvRef, Unit))
    let r = Expr::Apply(
        Expr::Apply(
            Expr::EnvRef("abc".to_string()).boxed(),
            Expr::Int(123).boxed(),
        ).boxed(),
        Expr::Apply(
            Expr::Apply(
                Expr::EnvRef("add".to_string()).boxed(),
                Expr::Int(123).boxed(),
            ).boxed(),
            Expr::Int(456).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    assert_eq!(parse_expr("abc 123 (add 123 456)"), r);
    assert_eq!(parse_expr("abc ((123)) (((add 123 456)))"), r);
    assert_eq!(parse_expr("(((abc (((123))) (((add (((123))) (((456)))))))))"), r);
}

#[test]
fn test_parse_expr_apply_part7() {
    // Apply(EnvRef, Apply(EnvRef, Unit))
    let r = Expr::Apply(
        Expr::Apply(
            Expr::EnvRef("abc".to_string()).boxed(),
            Expr::Apply(
                Expr::Apply(
                    Expr::EnvRef("add".to_string()).boxed(),
                    Expr::Int(123).boxed(),
                ).boxed(),
                Expr::Int(456).boxed(),
            ).boxed(),
        ).boxed(),
        Expr::Int(123).boxed(),
    );
    let r = Some(r);

    assert_eq!(parse_expr("abc (add 123 456) 123"), r);
    assert_eq!(parse_expr("abc (((add 123 456))) ((123))"), r);
    assert_eq!(parse_expr("(((abc (((add (((123))) (((456)))))) (((123))))))"), r);
}

#[test]
fn test_parse_expr_cond_part1() {
    // Cond(EnvRef, Int, Unit)
    let r = Expr::Cond(
        Expr::EnvRef("abc".to_string()).boxed(),
        Expr::Int(123).boxed(),
        Expr::Unit.boxed(),
    );
    let r = Some(r);

    assert_eq!(parse_expr("if abc then 123 else ()"), r);
    assert_eq!(parse_expr("if ((abc)) then ((123)) else ((()))"), r);
    assert_eq!(parse_expr("(if (((abc))) then (((123))) else (((()))))"), r);
    assert_eq!(parse_expr("(((if (((abc))) then (((123))) else (((()))))))"), r);
}

#[test]
fn test_parse_expr_cond_part2() {
    // Cond(a, a, a)
    // while: a = Cond(EnvRef, Apply(Int, Unit), Int)
    let e = Expr::Cond(
        Expr::EnvRef("abc".to_string()).boxed(),
        Expr::Apply(
            Expr::Int(123).boxed(),
            Expr::Unit.boxed(),
        ).boxed(),
        Expr::Int(456).boxed(),
    );
    let r = Some(Expr::Cond(
        e.clone().boxed(),
        e.clone().boxed(),
        e.clone().boxed(),
    ));

    let e = "if abc then 123 () else 456";
    let seq = &format!("if {} then {} else {}", e, e, e);
    assert_eq!(parse_expr(seq), r);
    let e = "if abc then (123 ()) else 456";
    let seq = &format!("if {} then {} else {}", e, e, e);
    assert_eq!(parse_expr(seq), r);
    let e = "(((if ((abc)) then ((123 ())) else ((456)))))";
    let seq = &format!("if {} then {} else {}", e, e, e);
    assert_eq!(parse_expr(seq), r);
}

#[test]
fn test_parse_expr_cond_part3() {
    // Cond(b, b, b)
    // while: a = Cond(Apply(Int, Unit), Int, EnvRef)
    // while: b = Cond(a, a, a)
    let a = Expr::Cond(
        Expr::Apply(
            Expr::Int(123).boxed(),
            Expr::Unit.boxed(),
        ).boxed(),
        Expr::Int(123).boxed(),
        Expr::EnvRef("abc".to_string()).boxed(),
    );
    let b = Expr::Cond(
        a.clone().boxed(),
        a.clone().boxed(),
        a.clone().boxed(),
    );
    let r = Expr::Cond(
        b.clone().boxed(),
        b.clone().boxed(),
        b.clone().boxed(),
    );
    let r = Some(r);

    let a = "if 123 () then 123 else abc";
    let b = &format!("if {} then {} else {}", a, a, a);
    let seq = &format!("if {} then {} else {}", b, b, b);
    assert_eq!(parse_expr(seq), r);
    let a = "(((if (((123 ()))) then (((123))) else (((abc))))))";
    let b = &format!("(((if {} then {} else {})))", a, a, a);
    let seq = &format!("if {} then {} else {}", b, b, b);
    assert_eq!(parse_expr(seq), r);
}

#[test]
fn test_parse_expr_cond_part4() {
    // Cond(b, b, b)
    // while: a = Cond(Apply(Int, Unit), Int, EnvRef)
    // while: b = Cond(a, a, a)
    let a = Expr::Cond(
        Expr::Apply(
            Expr::Int(123).boxed(),
            Expr::Unit.boxed(),
        ).boxed(),
        Expr::Int(123).boxed(),
        Expr::EnvRef("abc".to_string()).boxed(),
    );
    let b = Expr::Cond(
        a.clone().boxed(),
        a.clone().boxed(),
        a.clone().boxed(),
    );
    let r = Expr::Cond(
        b.clone().boxed(),
        b.clone().boxed(),
        b.clone().boxed(),
    );
    let r = Some(r);

    let a = "(((if (((123 ()))) then (((123))) else (((abc))))))";
    let b = &format!("(((if ((({}))) then ((({}))) else {})))", a, a, a);
    let seq = &format!("(((if ((({}))) then {} else ((({}))))))", b, b, b);
    assert_eq!(parse_expr(seq), r);
}

#[test]
fn test_parse_closure_part1() {
    let r = Expr::Closure(
        "a".to_string(),
        Expr::Apply(
            Expr::Apply(
                Expr::EnvRef("add".to_string()).boxed(),
                Expr::Int(123).boxed(),
            ).boxed(),
            Expr::Unit.boxed(),
        ).boxed(),
    );
    let r = Some(r);

    let seq = "a -> add 123 ()";
    assert_eq!(parse_expr(seq), r);
    let seq = "(a -> (add (123) (())))";
    assert_eq!(parse_expr(seq), r);
    let seq = "(((a -> ((((add 123)) ((())))))))";
    assert_eq!(parse_expr(seq), r);
}

#[test]
fn test_parse_closure_part2() {
    let r = Expr::Closure(
        "a".to_string(),
        Expr::Closure(
            "b".to_string(),
            Expr::Closure(
                "c".to_string(),
                Expr::Apply(
                    Expr::Apply(
                        Expr::EnvRef("add".to_string()).boxed(),
                        Expr::Apply(
                            Expr::Apply(
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
    assert_eq!(parse_expr(seq), r);
    let seq = "((a -> ((b -> ((c -> ((add (((add (a) (b)))) (c)))))))))";
    assert_eq!(parse_expr(seq), r);
    let seq = "((((((a))) -> (((b -> (((c))) -> (((add))) (add a b) c))))))";
    assert_eq!(parse_expr(seq), r);
}

#[test]
fn test_parse_closure_part3() {
    let r = Expr::Closure(
        "aaa".to_string(),
        Expr::Closure(
            "bbb".to_string(),
            Expr::Closure(
                "ccc".to_string(),
                Expr::Apply(
                    Expr::Apply(
                        Expr::EnvRef("add".to_string()).boxed(),
                        Expr::Apply(
                            Expr::Apply(
                                Expr::EnvRef("add".to_string()).boxed(),
                                Expr::EnvRef("aaa".to_string()).boxed(),
                            ).boxed(),
                            Expr::Int(123).boxed(),
                        ).boxed(),
                    ).boxed(),
                    Expr::EnvRef("ccc".to_string()).boxed(),
                ).boxed(),
            ).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    let seq = "aaa -> bbb -> ccc -> add (add aaa 123) ccc";
    assert_eq!(parse_expr(seq), r);
    let seq = "(((aaa -> ((bbb -> (ccc -> ((((((add (add aaa 123)))) ccc)))))))))";
    assert_eq!(parse_expr(seq), r);
    let seq = "(((aaa -> (((((bbb))) -> (((ccc)) -> ((((((add (add (((aaa))) 123)))) ccc)))))))))";
    assert_eq!(parse_expr(seq), r);
}

#[test]
fn test_parse_struct_part1() {
    let r = Expr::Struct(vec![
        ("a".to_string(), Expr::Int(123)),
        ("ab".to_string(), Expr::EnvRef("ref".to_string())),
        ("abc".to_string(), Expr::Unit),
    ]);
    let r = Some(r);

    let seq = "{ a = 123, ab = ref, abc = () }";
    assert_eq!(parse_expr(seq), r);
    let seq = "{ a = 123, ab = ref, abc = (),}";
    assert_eq!(parse_expr(seq), r);
    let seq = "(({ a = (((123))), ab = (((ref))), abc = ((())) }))";
    assert_eq!(parse_expr(seq), r);
    let seq = "(({ a = (((123))), ab = (((ref))), abc = ((())),}))";
    assert_eq!(parse_expr(seq), r);
}

#[test]
fn test_parse_struct_part2() {
    let a = Expr::Struct(vec![
        ("abc".to_string(),
         Expr::Struct(vec![
             ("efg".to_string(), Expr::Cond(
                 Expr::Int(123).boxed(),
                 Expr::Unit.boxed(),
                 Expr::Int(0).boxed(),
             ))
         ])),
        ("x".to_string(), Expr::Int(1)),
    ]);
    let f = Expr::Closure(
        "x".to_string(),
        Expr::Closure(
            "y".to_string(),
            Expr::Apply(
                Expr::Apply(
                    Expr::EnvRef("add".to_string()).boxed(),
                    Expr::EnvRef("x".to_string()).boxed(),
                ).boxed(),
                Expr::EnvRef("y".to_string()).boxed(),
            ).boxed(),
        ).boxed(),
    );
    let r = Expr::Struct(vec![
        ("a".to_string(), a),
        ("ab".to_string(), Expr::Apply(
            Expr::EnvRef("neg".to_string()).boxed(),
            Expr::Int(1).boxed(),
        )),
        ("f".to_string(), f),
    ]);
    let r = Some(r);

    let seq =
        "{ \
               a = { abc = { efg = if 123 then () else 0 }, x = 1 }, \
               ab = neg 1, \
               f = (x -> y -> add x y) \
             }";
    assert_eq!(parse_expr(seq), r);
    let seq =
        "((({ \
                  a = ((({ abc = { efg = if 123 then ((())) else 0 }, x = 1 }))), \
                  ab = (((neg))) 1, \
                  f = (x -> y -> add x y) \
            })))";
    assert_eq!(parse_expr(seq), r);
    let seq =
        "((({ \
                  (((a))) = ((({ abc = { efg = if (((123))) then ((())) else 0 }, x = (((1))) }))), \
                  (((ab))) = ((((((neg))) (((1)))))), \
                  (((f))) = (x -> (((y -> add x y)))) \
            })))";
    assert_eq!(parse_expr(seq), r);
}

#[test]
fn test_parse_match_part1() {
    let r = Expr::Match(
        Expr::EnvRef("x".to_string()).boxed(),
        vec![
            (Expr::Int(1),
             Expr::Apply(
                 Expr::EnvRef("neg".to_string()).boxed(),
                 Expr::Int(1).boxed(),
             )),
            (Expr::Int(2),
             Expr::Cond(
                 Expr::EnvRef("abc".to_string()).boxed(),
                 Expr::Unit.boxed(),
                 Expr::Int(0).boxed(),
             )),
            (Expr::Struct(vec![
                ("a".to_string(), Expr::Int(1)),
                ("b".to_string(), Expr::Discard),
                ("c".to_string(), Expr::Int(3)),
            ]),
             Expr::Int(0)),
            (Expr::Discard,
             Expr::Unit),
        ],
    );
    let r = Some(r);

    let seq =
        "match x with\
             | 1 -> neg 1\
             | 2 -> if abc then () else 0\
             | { a = 1, b = _, c = 3 } -> 0\
             | _ -> ()";
    assert_eq!(parse_expr(seq), r);
    let seq =
        "(((\
               match x with\
               | (((1))) -> (((neg 1)))\
               | (((2))) -> (((if (((abc))) then (((()))) else (((0))))))\
               | ((({ a = (((1))), b = (((_))), c = (((3))) }))) -> 0\
               | (((_))) -> (((())))\
             )))";
    assert_eq!(parse_expr(seq), r);
}

#[test]
fn test_parse_match_part2() {
    let r = Expr::Match(
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
    );
    let r = Some(r);

    let seq =
        "match x with\
             | 1 -> if a then b else c\
             | v -> a -> b -> add a b\
             | { a = _, b = { foo = _, bar = _ }, c = 3 } -> \
                 { x = 123, y = c }\
             | _ -> \
                match y with\
                | 1 -> ()\
                | () -> \
                     a -> b -> \
                       (\
                       match z with\
                       | _ -> 114514\
                       | a -> x -> y -> add () y\
                       )\
                | _ -> baz";

    assert_eq!(parse_expr(seq), r);

    let seq =
        "(((\
            match (((x))) with\
            | 1 -> if a then b else c\
            | (((v))) -> a -> b -> (((add a b)))\
            | { a = (((_))), b = { foo = (((_))), bar = (((_))) }, c = 3 } -> \
                ((({ x = (((123))), y = c })))\
            | (((_))) -> \
               (((\
               match y with\
               | 1 -> ()\
               | () -> \
                    (((\
                    a -> b -> \
                      (((\
                      match (((z))) with\
                      | (((_))) -> 114514\
                      | (((a))) -> \
                        (((\
                        (((x))) -> (((y))) -> (((add () y)))\
                        )))\
                      )))\
                    )))\
               | _ -> baz\
               )))\
             )))";

    assert_eq!(parse_expr(seq), r);
}

#[test]
fn test_parse_let_part1() {
    let r = Expr::Let(
        "a".to_string(),
        Expr::Int(123).boxed(),
        Expr::Apply(
            Expr::Apply(
                Expr::EnvRef("add".to_string()).boxed(),
                Expr::EnvRef("a".to_string()).boxed(),
            ).boxed(),
            Expr::Int(456).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    let seq = "let a = 123 in add a 456";
    assert_eq!(parse_expr(seq), r);
    let seq = "let a = 123,in add a 456";
    assert_eq!(parse_expr(seq), r);
    let seq = "(((let (((a))) = (((123))) in (((add a (((456)))))))))";
    assert_eq!(parse_expr(seq), r);
    let seq = "(((let (((a))) = (((123))),in (((add a (((456)))))))))";
    assert_eq!(parse_expr(seq), r);
}

#[test]
fn test_parse_let_part2() {
    let r = Expr::Let(
        "a".to_string(),
        Expr::Int(123).boxed(),
        Expr::Let(
            "b".to_string(),
            Expr::Apply(
                Expr::Apply(
                    Expr::EnvRef("add".to_string()).boxed(),
                    Expr::EnvRef("c".to_string()).boxed(),
                ).boxed(),
                Expr::EnvRef("d".to_string()).boxed(),
            ).boxed(),
            Expr::Apply(
                Expr::Apply(
                    Expr::EnvRef("add".to_string()).boxed(),
                    Expr::Unit.boxed(),
                ).boxed(),
                Expr::Int(456).boxed(),
            ).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    let seq = "let a = 123, b = add c d in add () 456";
    assert_eq!(parse_expr(seq), r);
}

#[test]
fn test_parse_let_part3() {
    let r = Expr::Let(
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
    assert_eq!(parse_expr(seq), r);
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
    assert_eq!(parse_expr(seq), r);
}
