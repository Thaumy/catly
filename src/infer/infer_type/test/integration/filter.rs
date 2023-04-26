use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#fn::has_type;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::test::integration::get_std_code;
use crate::infer::infer_type::test::parse_env;
use crate::{bool_type, closure_type, int_type};

fn gen_env<'t>() -> (TypeEnv, ExprEnv<'t>) {
    let seq = get_std_code() +
        "
        # 1
        def filter1: (Int -> Bool) -> IntList -> IntList =
            p -> list ->
                match list with
                | ({ head = head, tail = tail }: IntCons) ->
                    if p head then
                        intCons head (filter1 p tail)
                    else
                        filter1 p tail
                | (_: EmptyList) -> emptyList
        # 2
        def filter2 =
            # 对于 list 的类型标注是必须的, 因为 SumType 是不封闭的
            p -> list ->
                match (list: IntList) with
                | ({ head = head, tail = tail }: IntCons) ->
                    if p head then
                        (intCons head (filter2 p tail)): IntList
                    else
                        filter2 p tail
                | (_: EmptyList) -> emptyList
        ";
    parse_env(&seq)
}

fn target_type() -> InferTypeRet {
    has_type(closure_type!(
        closure_type!(int_type!(), bool_type!()),
        closure_type!(
            Type::NamelyType("IntList".to_string()),
            Type::NamelyType("IntList".to_string())
        )
    ))
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("filter1")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_eq!(expr_type, target_type())
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("filter2")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_eq!(expr_type, target_type())
}
