use crate::env::r#type::type_env::TypeEnv;
use crate::get_type::r#type::type_miss_match::TypeMissMatch;
use crate::get_type::r#type::GetTypeReturn;
use crate::infra::alias::MaybeType;
use crate::infra::r#box::Ext;
use crate::parser::r#type::r#type::Type;
use crate::unify::lift_or_left;
use crate::{has_type, require_info};

pub fn case_t(
    type_env: &TypeEnv,
    expect_type: &MaybeType,
    input_name: &Option<String>,
    input_type: MaybeType,
    output_expr_type: Type
) -> GetTypeReturn {
    let base = match input_type {
        Some(input_type) => Type::ClosureType(
            input_type.clone().boxed(),
            output_expr_type.boxed()
        ),
        // 输入类型自由, 而 output_expr_type 不需要约束, 说明不需要输入类型
        // 因为类型和值绑定, 所以 output_expr 和输入无关, 实际上这和弃元输入值等效
        // 缺乏推导出输入类型的信息
        None =>
            return match input_name {
                Some(input_name) =>
                    require_info!(input_name.to_string()),
                None => require_info!("_ (closure input)".to_string())
            },
    };

    // Lift inferred ClosureType to t
    match lift_or_left(type_env, &base, expect_type) {
        Some(t) => has_type!(t),
        None => TypeMissMatch::of_type(
            &base,
            &expect_type.clone().unwrap()
        )
        .into()
    }
}
