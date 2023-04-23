mod case_rc;
mod case_t;

use std::ops::Deref;

use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::infer_type::case::closure::case_rc::case_rc;
use crate::infer_type::case::closure::case_t::case_t;
use crate::infer_type::r#fn::destruct_namely_type;
use crate::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer_type::r#type::require_info::RequireInfo;
use crate::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::Type;

// TODO: 外部环境约束同层传播完备性
pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &MaybeType,
    input_name: &Option<String>,
    input_type: &MaybeType,
    output_expr: &Expr
) -> InferTypeRet {
    // Destruct expect_type to ClosureType
    let (expect_input_type, expect_output_type) = match expect_type {
        Some(expect_type) =>
        // 允许将 ClosureType 提升到基于它的 NamelyType
        // 换言之, 如果 expect_type 是基于 ClosureType 的, 那么它也能够通过类型检查
            match destruct_namely_type(type_env, expect_type) {
                Some(t) => match t {
                    Type::ClosureType(
                        expect_input_type,
                        expect_output_type
                    ) => (
                        expect_input_type
                            .deref()
                            .clone()
                            .some(),
                        expect_output_type
                            .deref()
                            .clone()
                            .some()
                    ),
                    Type::PartialClosureType(expect_input_type) => (
                        expect_input_type
                            .deref()
                            .clone()
                            .some(),
                        None
                    ),

                    _ => return TypeMissMatch::of(&format!("{expect_type:?} <> ClosureType/PartialClosureType")).into()
                },
                _ => return TypeMissMatch::of(&format!("{expect_type:?} not found in type env")).into()
            },
        _ => (None, None)
    };

    // Hint input_type
    let input_type = match input_type {
        None => expect_input_type,
        _ => input_type.clone()
    };

    // Inject parameter to env
    let expr_env = match input_name {
        Some(input_name) => expr_env.extend_new(
            input_name.clone(),
            input_type.clone(),
            None
        ),
        _ => expr_env.clone()
    };

    // Hint and get output_expr_type
    let output_expr_type = output_expr
        .with_optional_fallback_type(&expect_output_type)
        .infer_type(type_env, &expr_env);

    // 此处并不将 output_expr_type 与 hint 进行相容性判断
    // 因为这与 Closure 的类型提升规则相同, 稍后的类型提升会进行该工作
    // 而且提前返回带来的性能提升并不显著

    match output_expr_type {
        Quad::L(output_expr_type) => case_t(
            type_env,
            expect_type,
            input_name,
            input_type,
            output_expr_type
        ),

        Quad::ML(rc) => case_rc(
            type_env,
            expect_type,
            rc.r#type,
            rc.constraint,
            input_name,
            input_type
        ),

        Quad::MR(ri) if let Some(input_name) = input_name =>
            RequireInfo::of(
                &ri.ref_name,
                ri.constraint.exclude_new(input_name.as_str())
            )
            .into(),

        // infer_type 不能推导出输出类型(即便进行了类型提示), 或推导错误
        // 推导错误是由类型不匹配导致的, 这种错误无法解决
        // 不能推导出输出类型是由缺乏类型信息导致的
        // 因为 Closure 不存在可以推导输出类型的第二个表达式, 所以不适用于旁路类型推导
        r => r
    }
}