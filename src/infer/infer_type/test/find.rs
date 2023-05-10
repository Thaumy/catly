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
        def find1: Int -> IntList -> Bool =
            n -> list ->
                match list with
                | ({ head = head, tail = tail }: IntCons) ->
                    if eq head n then
                        true
                    else
                        find1 n tail
                | (_: EmptyList) -> false
        # 2
        def find2 =
            # 对于 list 的类型标注是必须的, 因为 SumType 是不封闭的
            # 分支中的 Bool 标注同理
            n -> (list: IntList) -> (
                match list with
                | ({ head = head, tail = tail }: IntCons) ->
                    if eq head n then
                        true
                    else
                        find2 n tail
                | (_: EmptyList) -> false
            ): Bool
        ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("find1")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(
        int_type!(),
        closure_type!(
            Type::NamelyType("IntList".to_string()),
            bool_type!()
        )
    );
    check_has_type!(infer_result, t)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("find2")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(
        int_type!(),
        closure_type!(
            Type::NamelyType("IntList".to_string()),
            bool_type!()
        )
    );
    check_has_type!(infer_result, t)
}
