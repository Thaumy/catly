use crate::parser::expr::Expr;
use crate::parser::expr::test::f;
use crate::parser::infra::option::AnyExt;
use crate::parser::infra::r#box::Ext;
use crate::parser::r#type::Type;

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
                Expr::EnvRef(None, "add".to_string()).boxed(),
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
                        Expr::EnvRef(None, "add".to_string()).boxed(),
                        Expr::Apply(
                            None,
                            Expr::Apply(
                                None,
                                Expr::EnvRef(None, "add".to_string()).boxed(),
                                Expr::EnvRef(None, "a".to_string()).boxed(),
                            ).boxed(),
                            Expr::EnvRef(None, "b".to_string()).boxed(),
                        ).boxed(),
                    ).boxed(),
                    Expr::EnvRef(None, "c".to_string()).boxed(),
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
                        Expr::EnvRef(None, "add".to_string()).boxed(),
                        Expr::Apply(
                            None,
                            Expr::Apply(
                                None,
                                Expr::EnvRef(None, "add".to_string()).boxed(),
                                Expr::EnvRef(None, "aaa".to_string()).boxed(),
                            ).boxed(),
                            Expr::Int(None, 123).boxed(),
                        ).boxed(),
                    ).boxed(),
                    Expr::EnvRef(None, "ccc".to_string()).boxed(),
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
fn test_parse_closure_part4() {
    let r = Expr::Closure(
        None,
        "a".to_string(),
        Type::TypeEnvRef("A".to_string()).some(),
        Expr::Closure(
            None,
            "b".to_string(),
            Type::TypeEnvRef("B".to_string()).some(),
            Expr::Closure(
                None,
                "c".to_string(),
                Type::TypeEnvRef("C".to_string()).some(),
                Expr::Apply(
                    Type::TypeEnvRef("Int".to_string()).some(),
                    Expr::Apply(
                        None,
                        Expr::EnvRef(None, "add".to_string()).boxed(),
                        Expr::Int(
                            Type::TypeEnvRef("Int".to_string()).some(),
                            123,
                        ).boxed(),
                    ).boxed(),
                    Expr::EnvRef(None, "ccc".to_string()).boxed(),
                ).boxed(),
            ).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    let seq = "(a: A) -> (b: B) -> (c: C) -> (add (123: Int) ccc): Int";
    assert_eq!(f(seq), r);
    let seq = "(a: A) -> (((b: B) -> ((((c: C) -> (((add (123: Int) ccc): Int)))))))";
    assert_eq!(f(seq), r);
}
