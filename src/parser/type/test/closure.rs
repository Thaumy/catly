use crate::infra::r#box::Ext;
use crate::parser::r#type::test::f;
use crate::{closure_type, namely_type};

#[test]
fn test_part1() {
    let r = closure_type!(namely_type!("T"), namely_type!("TList"));
    let r = Some(r);

    let seq = "T -> TList";
    assert_eq!(f(seq), r);
    let seq = "((((((T))) -> (((TList))))))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_part2() {
    let r = closure_type!(
        namely_type!("T"),
        closure_type!(namely_type!("U"), namely_type!("TUEither"))
    );
    let r = Some(r);

    let seq = "T -> U ->  TUEither";
    assert_eq!(f(seq), r);
    let seq = "(((T -> (((U -> (((TUEither)))))))))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_part3() {
    let r = closure_type!(
        closure_type!(namely_type!("T"), namely_type!("U")),
        namely_type!("TUEither")
    );
    let r = Some(r);

    let seq = "(T -> U) -> TUEither";
    assert_eq!(f(seq), r);
    let seq = "((((((T -> U))) -> (((TUEither))))))";
    assert_eq!(f(seq), r);
}
