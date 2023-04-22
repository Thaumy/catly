use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::infer_type::r#fn::has_type;
use crate::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer_type::test::integration::get_std_code;
use crate::infer_type::test::parse_env;
use crate::{closure_type, int_type};

fn gen_env<'t>() -> (TypeEnv, ExprEnv<'t>) {
    let seq = get_std_code() +
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
    parse_env(&seq)
}

fn target_type() -> InferTypeRet {
    has_type(closure_type!(
        closure_type!(int_type!(), int_type!()),
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
        .get_ref("map1")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_eq!(expr_type, target_type())
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("map2")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_eq!(expr_type, target_type())
}
