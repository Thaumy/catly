use crate::infer::env::r#macro::closure_type;
use crate::infer::env::r#macro::int_type;
use crate::infer::env::r#macro::namely_type;
use crate::infra::option::WrapOption;
use crate::infra::rc::RcAnyExt;
use crate::parser::expr::r#type::Expr;
use crate::parser::expr::test::f;

#[test]
fn test_part1() {
    let r = Expr::Closure(
        None,
        "a".to_string().wrap_some(),
        None,
        Expr::Apply(
            None,
            Expr::Apply(
                None,
                Expr::EnvRef(None, "add".to_string()).rc(),
                Expr::Int(None, 123).rc()
            )
            .rc(),
            Expr::Unit(None).rc()
        )
        .rc()
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
fn test_part2() {
    let r = Expr::Closure(
        None,
        "a".to_string().wrap_some(),
        None,
        Expr::Closure(
            None,
            "b".to_string().wrap_some(),
            None,
            Expr::Closure(
                None,
                None,
                None,
                Expr::Apply(
                    None,
                    Expr::Apply(
                        None,
                        Expr::EnvRef(None, "add".to_string()).rc(),
                        Expr::Apply(
                            None,
                            Expr::Apply(
                                None,
                                Expr::EnvRef(None, "add".to_string())
                                    .rc(),
                                Expr::EnvRef(None, "a".to_string())
                                    .rc()
                            )
                            .rc(),
                            Expr::EnvRef(None, "b".to_string()).rc()
                        )
                        .rc()
                    )
                    .rc(),
                    Expr::EnvRef(None, "c".to_string()).rc()
                )
                .rc()
            )
            .rc()
        )
        .rc()
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
fn test_part3() {
    let r = Expr::Closure(
        None,
        "aaa".to_string().wrap_some(),
        None,
        Expr::Closure(
            None,
            None,
            None,
            Expr::Closure(
                None,
                "ccc".to_string().wrap_some(),
                None,
                Expr::Apply(
                    None,
                    Expr::Apply(
                        None,
                        Expr::EnvRef(None, "add".to_string()).rc(),
                        Expr::Apply(
                            None,
                            Expr::Apply(
                                None,
                                Expr::EnvRef(None, "add".to_string())
                                    .rc(),
                                Expr::EnvRef(None, "aaa".to_string())
                                    .rc()
                            )
                            .rc(),
                            Expr::Int(None, 123).rc()
                        )
                        .rc()
                    )
                    .rc(),
                    Expr::EnvRef(None, "ccc".to_string()).rc()
                )
                .rc()
            )
            .rc()
        )
        .rc()
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
fn test_part4() {
    let r = Expr::Closure(
        None,
        None,
        namely_type!("A").wrap_some(),
        Expr::Closure(
            None,
            "b".to_string().wrap_some(),
            namely_type!("B").wrap_some(),
            Expr::Closure(
                None,
                "c".to_string().wrap_some(),
                namely_type!("C").wrap_some(),
                Expr::Apply(
                    int_type!().wrap_some(),
                    Expr::Apply(
                        None,
                        Expr::EnvRef(None, "add".to_string()).rc(),
                        Expr::Int(int_type!().wrap_some(), 123).rc()
                    )
                    .rc(),
                    Expr::EnvRef(None, "ccc".to_string()).rc()
                )
                .rc()
            )
            .rc()
        )
        .rc()
    );
    let r = Some(r);

    let seq =
        "(_: A) -> (b: B) -> (c: C) -> (add (123: Int) ccc): Int";
    assert_eq!(f(seq), r);
    let seq = "(_: A) -> (((b: B) -> ((((c: C) -> (((add (123: Int) ccc): Int)))))))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_part5() {
    let r = Expr::Closure(
        closure_type!(namely_type!("A"), namely_type!("B"))
            .wrap_some(),
        "a".to_string().wrap_some(),
        None,
        Expr::Closure(
            None,
            "b".to_string().wrap_some(),
            None,
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "add".to_string()).rc(),
                    Expr::EnvRef(None, "a".to_string()).rc()
                )
                .rc(),
                Expr::EnvRef(None, "b".to_string()).rc()
            )
            .rc()
        )
        .rc()
    );
    let r = Some(r);

    let seq = "(a -> b -> add a b): A -> B";
    assert_eq!(f(seq), r);
}
