use crate::infra::option::AnyExt;
use crate::infra::r#box::Ext;
use crate::int_type;
use crate::parser::expr::test::f;
use crate::parser::expr::Expr;
use crate::parser::r#type::Type;

#[test]
fn test_parse_closure_part1() {
    let r = Expr::Closure(
        None,
        "a".to_string().some(),
        None,
        Expr::Apply(
            None,
            Expr::Apply(
                None,
                Expr::EnvRef(None, "add".to_string()).boxed(),
                Expr::Int(None, 123).boxed()
            )
            .boxed(),
            Expr::Unit(None).boxed()
        )
        .boxed()
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
        "a".to_string().some(),
        None,
        Expr::Closure(
            None,
            "b".to_string().some(),
            None,
            Expr::Closure(
                None,
                None,
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
                                Expr::EnvRef(None, "add".to_string())
                                    .boxed(),
                                Expr::EnvRef(None, "a".to_string())
                                    .boxed()
                            )
                            .boxed(),
                            Expr::EnvRef(None, "b".to_string())
                                .boxed()
                        )
                        .boxed()
                    )
                    .boxed(),
                    Expr::EnvRef(None, "c".to_string()).boxed()
                )
                .boxed()
            )
            .boxed()
        )
        .boxed()
    );
    let r = Some(r);

    let seq = "a -> b -> _ -> add (add a b) c";
    assert_eq!(f(seq), r);
    let seq =
        "((a -> ((b -> ((_ -> ((add (((add (a) (b)))) (c)))))))))";
    assert_eq!(f(seq), r);
    let seq = "((((((a))) -> (((b -> (((_))) -> (((add))) (add a b) c))))))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_closure_part3() {
    let r = Expr::Closure(
        None,
        "aaa".to_string().some(),
        None,
        Expr::Closure(
            None,
            None,
            None,
            Expr::Closure(
                None,
                "ccc".to_string().some(),
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
                                Expr::EnvRef(None, "add".to_string())
                                    .boxed(),
                                Expr::EnvRef(None, "aaa".to_string())
                                    .boxed()
                            )
                            .boxed(),
                            Expr::Int(None, 123).boxed()
                        )
                        .boxed()
                    )
                    .boxed(),
                    Expr::EnvRef(None, "ccc".to_string()).boxed()
                )
                .boxed()
            )
            .boxed()
        )
        .boxed()
    );
    let r = Some(r);

    let seq = "aaa -> _ -> ccc -> add (add aaa 123) ccc";
    assert_eq!(f(seq), r);
    let seq = "(((aaa -> ((_ -> (ccc -> ((((((add (add aaa 123)))) ccc)))))))))";
    assert_eq!(f(seq), r);
    let seq = "(((aaa -> (((((_))) -> (((ccc)) -> ((((((add (add (((aaa))) 123)))) ccc)))))))))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_closure_part4() {
    let r = Expr::Closure(
        None,
        None,
        Type::TypeEnvRef("A".to_string()).some(),
        Expr::Closure(
            None,
            "b".to_string().some(),
            Type::TypeEnvRef("B".to_string()).some(),
            Expr::Closure(
                None,
                "c".to_string().some(),
                Type::TypeEnvRef("C".to_string()).some(),
                Expr::Apply(
                    int_type!().some(),
                    Expr::Apply(
                        None,
                        Expr::EnvRef(None, "add".to_string()).boxed(),
                        Expr::Int(int_type!().some(), 123).boxed()
                    )
                    .boxed(),
                    Expr::EnvRef(None, "ccc".to_string()).boxed()
                )
                .boxed()
            )
            .boxed()
        )
        .boxed()
    );
    let r = Some(r);

    let seq =
        "(_: A) -> (b: B) -> (c: C) -> (add (123: Int) ccc): Int";
    assert_eq!(f(seq), r);
    let seq = "(_: A) -> (((b: B) -> ((((c: C) -> (((add (123: Int) ccc): Int)))))))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_closure_part5() {
    let r = Expr::Closure(
        Type::ClosureType(
            Type::TypeEnvRef("A".to_string()).boxed(),
            Type::TypeEnvRef("B".to_string()).boxed()
        )
        .some(),
        "a".to_string().some(),
        None,
        Expr::Closure(
            None,
            "b".to_string().some(),
            None,
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "add".to_string()).boxed(),
                    Expr::EnvRef(None, "a".to_string()).boxed()
                )
                .boxed(),
                Expr::EnvRef(None, "b".to_string()).boxed()
            )
            .boxed()
        )
        .boxed()
    );
    let r = Some(r);

    let seq = "(a -> b -> add a b): A -> B";
    assert_eq!(f(seq), r);
}
