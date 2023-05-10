use std::rc::Rc;

use crate::eval::std::std_code;
use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::parse_to_env;
use crate::infer::env::r#macro::bool_type;
use crate::infer::env::r#macro::closure_type;
use crate::infer::env::r#macro::int_type;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::test::check_has_type;

fn gen_env<'t>() -> (TypeEnv<'t>, Rc<ExprEnv>) {
    let seq = std_code().to_owned() +
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
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("filter1")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(
        closure_type!(int_type!(), bool_type!()),
        closure_type!(
            Type::NamelyType("IntList".to_string()),
            Type::NamelyType("IntList".to_string())
        )
    );
    check_has_type!(infer_result, t)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("filter2")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(
        closure_type!(int_type!(), bool_type!()),
        closure_type!(
            Type::NamelyType("IntList".to_string()),
            Type::NamelyType("IntList".to_string())
        )
    );
    check_has_type!(infer_result, t)
}
