mod has_input_type;
mod no_input_type;
/*#[cfg(test)]
mod test;
*/
use std::ops::Deref;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::case::closure::has_input_type::has_input_type;
use crate::infer::infer_type::case::closure::no_input_type::no_input_type;
use crate::infer::infer_type::r#fn::destruct_namely_type;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::require_info::ReqInfo;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::option::OptionAnyExt;
use crate::infra::r#box::BoxAnyExt;
use crate::infra::triple::Triple;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::OptType;
use crate::parser::r#type::r#type::Type;

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &OptType,
    input_name: &Option<String>,
    input_type: &OptType,
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
                None => return TypeMissMatch::of(&format!("{expect_type:?} not found in type env")).into()
            },
        None => (None, None)
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

    // 此处并不将 output_expr_type 与 hint 进行相容性判断
    // 因为这与 Closure 的类型提升规则相同, 稍后的类型提升会进行该工作
    // 而且提前返回带来的性能提升并不显著
    match output_expr
        // Hint and get output_expr_type
        .with_opt_fallback_type(&expect_output_type)
        .infer_type(type_env, &expr_env)?
    {
        output_expr_type @ (Triple::L(_) | Triple::M(_)) => {
            let (output_expr_type, constraint_acc, typed_output_expr) =
                output_expr_type.unwrap_type_constraint_expr();

            let typed_expr_cons =
                |type_annot: Type,
                 input_type: Type| {
                    Expr::Closure(
                        type_annot.some(),
                        input_name.clone(),
                        input_type.clone().some(),
                        typed_output_expr
                            .clone()
                            .boxed()
                    )
                };

            match input_type {
                Some(input_type) => has_input_type(
                    type_env,
                    expect_type,
                    output_expr_type,
                    input_type,
                    typed_expr_cons
                )?
                .with_constraint_acc(constraint_acc),
                // 如果注入约束到环境, 此处还可从环境中寻找可能的输入类型(从而不必传递约束
                None => no_input_type(
                    type_env,
                    expect_type,
                    output_expr_type,
                    constraint_acc,
                    input_name,
                    typed_expr_cons
                )
            }
        }

        // infer_type 不能推导出输出类型(即便进行了类型提示), 但可以传播约束, 为下一轮推导提供信息
        // Closure 不存在可以推导输出类型的第二个表达式, 所以不适用于旁路类型推导
        Triple::R(ri) if let Some(input_name) = input_name => ReqInfo::of(
            &ri.ref_name,
            ri.constraint
                .exclude_new(input_name.as_str())
        )
        .into(),

        ri => ri.into()
    }
}
