use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::r#box::Ext;
use crate::parser::r#type::Type;
use crate::type_checker::env::type_env::TypeEnv;
use crate::type_checker::get_type::r#fn::with_constraint_lift_or_left;
use crate::type_checker::get_type::r#type::{
    GetTypeReturn,
    RequireConstraint
};

pub fn case_rc(
    rc: RequireConstraint,
    type_env: &TypeEnv,
    expect_type: &MaybeType,
    input_name: &Option<String>,
    input_type: MaybeType
) -> GetTypeReturn {
    // 因为需要判断约束是否包含输入, 所以需要匹配输入名
    let (base, left_constraint) = match input_name {
        Some(input_name) => match input_type {
            // 因为输入具名有类型, 所以约束不可能包含自输入
            // 换言之, 输入在推导 output_expr_type 之前就已经被约束了
            // 此时只需要提升类型, 并将约束传播
            Some(input_type) => {
                let base = Type::ClosureType(
                    input_type
                        .clone()
                        .boxed()
                        .some(),
                    rc.r#type.boxed().some()
                );

                (base, rc.constraint)
            }
            // 输入具名无类型
            // 如果约束包含了输入, 需把输入类型限定到约束目标, 并将其从约束列表中移除
            // 然后传播剩余约束(仍有剩余约束), 或返回确切类型(不存在剩余约束)
            None => {
                let input_type_constraint = rc
                    .constraint
                    .iter()
                    .rev()
                    .find(|(n, _)| n == input_name)
                    .map(|(_, t)| t);

                if let Some(input_type_constraint) =
                    input_type_constraint
                {
                    // 约束包含输入, 需要限定输入类型到约束目标并将其从约束列表中移除
                    // 将剩余约束过滤出来
                    let left_constraint = rc
                        .constraint
                        .iter()
                        .filter(|(n, _)| n != input_name)
                        .map(|x| x.clone())
                        .collect():
                        Vec<_>;

                    let base = Type::ClosureType(
                        input_type_constraint
                            .clone()
                            .boxed()
                            .some(),
                        rc.r#type.boxed().some()
                    );

                    (base, left_constraint)
                } else {
                    // 约束不包含输入, 只需将其传播
                    let base = Type::ClosureType(
                        None,
                        rc.r#type.boxed().some()
                    );

                    (base, rc.constraint)
                }
            }
        },
        None => {
            // 输入被弃元, 但 output_expr_type 需要约束
            // 说明约束目标是外层环境
            let base =
                Type::ClosureType(None, rc.r#type.boxed().some());

            (base, rc.constraint)
        }
    };

    with_constraint_lift_or_left(
        left_constraint,
        type_env,
        &base,
        expect_type
    )
}
