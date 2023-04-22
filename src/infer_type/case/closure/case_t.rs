use crate::env::r#type::type_env::TypeEnv;
use crate::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer_type::r#type::require_info::RequireInfo;
use crate::infra::alias::MaybeType;
use crate::infra::r#box::Ext;
use crate::parser::r#type::r#type::Type;

pub fn case_t(
    type_env: &TypeEnv,
    expect_type: &MaybeType,
    input_name: &Option<String>,
    input_type: MaybeType,
    output_expr_type: Type
) -> InferTypeRet {
    let base = match input_type {
        Some(input_type) => Type::ClosureType(
            input_type.clone().boxed(),
            output_expr_type.boxed()
        ),
        // 输入类型自由, 而 output_expr_type 不需要约束, 说明不需要输入类型
        // 因为类型和值绑定, 所以 output_expr 和输入无关, 实际上这和弃元输入值等效
        // 缺乏推导出输入类型的信息
        None => {
            let input_name = &input_name
                .clone()
                .unwrap_or("_ (closure input)".to_string());
            return RequireInfo::of(
                input_name,
                EnvRefConstraint::empty()
            )
            .into();
        }
    };

    // Lift inferred ClosureType to t
    InferTypeRet::from_auto_lift(type_env, &base, expect_type, None)
}
