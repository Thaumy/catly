use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::require_info::RequireInfo;
use crate::infra::option::OptionAnyExt;
use crate::infra::r#box::BoxAnyExt;
use crate::parser::r#type::r#type::OptType;
use crate::parser::r#type::r#type::Type;

pub fn case_t_rc(
    type_env: &TypeEnv,
    expect_type: &OptType,
    output_expr_type: Type,
    constraint_acc: EnvRefConstraint,
    input_name: &Option<String>,
    input_type: OptType
) -> InferTypeRet {
    let (input_type, constraint_acc) = match input_type {
        // 能够获取到输入类型
        Some(input_type) => (input_type, constraint_acc),
        // 尝试从获取 output_expr_type 产生的约束中恢复输入类型
        None => {
            let input_name = match input_name {
                Some(n) => n,
                // input_name 被弃元, 说明 output_expr_type 产生的约束全部作用于外层环境
                // 此时不可能确定输入类型
                None =>
                    return RequireInfo::of(
                        "_ (closure input)",
                        constraint_acc
                    )
                    .into(),
            };

            // 查找约束, 如果发现针对 input_name 的约束, 那么输入类型就可以确定
            match constraint_acc.find(input_name.as_str()) {
                Some(input_type_constraint) => {
                    // 约束包含输入, 需要限定输入类型到约束目标并将其从约束列表中移除
                    // 将剩余约束过滤出来
                    let left_constraint = constraint_acc
                        .exclude_new(input_name.as_str());

                    (input_type_constraint.clone(), left_constraint)
                }
                None =>
                // 约束不包含输入, 缺乏推导出输入类型的信息
                    return RequireInfo::of(
                        input_name,
                        constraint_acc
                    )
                    .into(),
            }
        }
    };

    let base = Type::ClosureType(
        input_type.clone().boxed(),
        output_expr_type.boxed()
    );

    InferTypeRet::from_auto_lift(
        type_env,
        &base,
        expect_type,
        constraint_acc.some()
    )
}
