use crate::env::r#type::type_env::TypeEnv;
use crate::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer_type::r#type::require_info::RequireInfo;
use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::r#box::Ext;
use crate::parser::r#type::r#type::Type;

pub fn case_rc(
    type_env: &TypeEnv,
    expect_type: &MaybeType,
    output_expr_type: Type,
    constraint_acc: EnvRefConstraint,
    input_name: &Option<String>,
    input_type: MaybeType
) -> InferTypeRet {
    // 因为需要判断约束是否包含输入, 所以需要匹配输入名
    let (base, constraint_acc) = match input_name {
        Some(input_name) => match input_type {
            // 因为输入具名有类型, 所以约束不可能包含自输入
            // 换言之, 输入在推导 output_expr_type 之前就已经被约束了
            // 此时只需要提升类型, 并将约束传播
            Some(input_type) => {
                let base = Type::ClosureType(
                    input_type.clone().boxed(),
                    output_expr_type.boxed()
                );

                (base, constraint_acc)
            }
            // 输入具名无类型
            // 如果约束包含了输入, 需把输入类型限定到约束目标, 并将其从约束列表中移除
            // 然后传播剩余约束(仍有剩余约束), 或返回确切类型(不存在剩余约束)
            None => {
                let input_type_constraint =
                    constraint_acc.find(input_name.as_str());

                if let Some(input_type_constraint) =
                    input_type_constraint
                {
                    // 约束包含输入, 需要限定输入类型到约束目标并将其从约束列表中移除
                    // 将剩余约束过滤出来
                    let left_constraint = constraint_acc
                        .exclude_new(input_name.as_str());

                    let base = Type::ClosureType(
                        input_type_constraint
                            .clone()
                            .boxed(),
                        output_expr_type.boxed()
                    );

                    (base, left_constraint)
                } else {
                    // 约束不包含输入, 缺乏推导出输入类型的信息
                    return RequireInfo::of(
                        input_name,
                        constraint_acc
                    )
                    .into();
                }
            }
        },
        // 输入被弃元, 说明 output_expr_type 产生的约束全部作用于外层环境
        None =>
            return RequireInfo::of(
                "_ (closure input)",
                constraint_acc
            )
            .into(),
    };

    InferTypeRet::from_auto_lift(
        type_env,
        &base,
        expect_type,
        constraint_acc.some()
    )
}
