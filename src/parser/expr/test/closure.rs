use crate::parser::expr::Expr;
use crate::parser::expr::test::f;
use crate::parser::infra::r#box::Ext;

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
