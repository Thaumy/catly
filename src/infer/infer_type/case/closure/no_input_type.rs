use crate::infer::env::TypeEnv;
use crate::infer::infer_type::EnvRefConstraint;
use crate::infer::infer_type::InferTypeRet;
use crate::infer::infer_type::ReqInfo;
use crate::infra::RcAnyExt;
use crate::infra::WrapOption;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::OptType;
use crate::parser::r#type::Type;

pub fn no_input_type<F>(
    type_env: &TypeEnv,
    expect_type: &OptType,
    output_expr_type: &Type,
    constraint_acc: EnvRefConstraint,
    input_name: &Option<String>,
    typed_expr_cons: F
) -> InferTypeRet
where
    F: Fn(Type, Type) -> Expr
{
    // 尝试从获取 output_expr_type 产生的约束中恢复输入类型
    let input_name = match input_name {
        Some(n) => n,
        // input_name 被弃元, 说明 output_expr_type 产生的约束全部作用于外层环境
        // 此时不可能确定输入类型
        None =>
            return ReqInfo::of("_ (closure input)", constraint_acc)
                .into(),
    };

    // 查找约束, 如果发现针对 input_name 的约束, 那么输入类型就可以确定
    match constraint_acc.find(input_name.as_str()) {
        // 约束包含输入, 需要限定输入类型到约束目标并将其从约束列表中移除
        Some(input_type_constraint) => {
            // 将剩余约束过滤出来
            let left_constraint =
                constraint_acc.exclude_new(input_name.as_str());

            let base = Type::ClosureType(
                input_type_constraint
                    .clone()
                    .rc(),
                output_expr_type.clone().rc()
            );

            InferTypeRet::from_auto_lift(
                type_env,
                &base,
                expect_type,
                left_constraint.wrap_some(),
                |t| typed_expr_cons(t, input_type_constraint.clone())
            )
        }
        // 约束不包含输入, 缺乏推导出输入类型的信息
        None => ReqInfo::of(input_name, constraint_acc).into()
    }
}
