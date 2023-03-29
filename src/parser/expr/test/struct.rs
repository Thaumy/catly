use crate::parser::expr::Expr;
use crate::parser::expr::test::f;
use crate::parser::infra::r#box::Ext;

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
               fun = x -> y -> add x y \
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
