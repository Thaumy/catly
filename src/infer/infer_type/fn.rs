use crate::infer::env::type_env::TypeEnv;
use crate::infra::option::WrapOption;
use crate::parser::r#type::r#type::Type;

pub fn destruct_namely_type(
    type_env: &TypeEnv,
    r#type: &Type
) -> Option<Type> {
    if r#type.is_primitive() {
        return r#type.clone().wrap_some();
    }

    match r#type {
        Type::NamelyType(type_name) => {
            let base_type = type_env.find_type(type_name.as_str())?;
            destruct_namely_type(type_env, &base_type)
        }
        x => x.clone().wrap_some()
    }
}
