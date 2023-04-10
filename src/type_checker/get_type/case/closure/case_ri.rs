use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::r#box::Ext;
use crate::parser::r#type::Type;
use crate::type_checker::env::type_env::TypeEnv;
use crate::type_checker::get_type::r#fn::lift_or_left;
use crate::type_checker::get_type::r#type::GetTypeReturn;
use crate::{has_type, type_miss_match};

pub fn case_ri(
    type_env: &TypeEnv,
    expect_type: &MaybeType,
    input_type: &MaybeType
) -> GetTypeReturn {
    // 输入和输出都是自由的, 它们可以变换到任何类型
    let base = match input_type {
        Some(input_type) => Type::ClosureType(
            input_type
                .clone()
                .boxed()
                .some(),
            None
        ),
        None => Type::ClosureType(None, None)
    };

    // Lift inferred ClosureType to t
    match lift_or_left(type_env, &base, expect_type) {
        Some(t) => has_type!(t),
        None => type_miss_match!()
    }
}
