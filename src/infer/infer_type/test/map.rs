use std::rc::Rc;

use crate::eval::std::std_code;
use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::parse_to_env;
use crate::infer::env::r#macro::closure_type;
use crate::infer::env::r#macro::int_type;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::test::check_has_type;

fn gen_env<'t>() -> (TypeEnv<'t>, Rc<ExprEnv>) {
    let seq = std_code().to_owned() +
        "
        # 1
        def map1: (Int -> Int) -> IntList -> IntList =
            f -> list ->
                match list with
                | ({ head = head, tail = tail }: IntCons) ->
                    intCons (f head) (map1 f tail)
                | (_: EmptyList) -> emptyList
        # 2
        def map2 =
            # 对于 list 的类型标注是必须的, 因为 SumType 是不封闭的
            # 分支中的 IntList 标注同理
            f -> (list: IntList) -> (
                match list with
                | ({ head = head, tail = tail }: IntCons) ->
                    intCons (f head) (map2 f tail)
                | (_: EmptyList) -> emptyList
            ): IntList
        ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_ref("map1")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(
        closure_type!(int_type!(), int_type!()),
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
        .get_ref("map2")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let t = closure_type!(
        closure_type!(int_type!(), int_type!()),
        closure_type!(
            Type::NamelyType("IntList".to_string()),
            Type::NamelyType("IntList".to_string())
        )
    );
    check_has_type!(infer_result, t)
}
