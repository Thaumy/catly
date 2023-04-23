use crate::env::r#type::type_env::TypeEnv;
use crate::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::parser::r#type::r#type::Type;

pub fn destruct_namely_type(
    type_env: &TypeEnv,
    r#type: &Type
) -> Option<Type> {
    if r#type.is_primitive() {
        return r#type.clone().some();
    }

    match r#type {
        Type::NamelyType(type_name) => {
            let base_type = type_env.find_type(type_name)?;
            destruct_namely_type(type_env, &base_type)
        }
        x => x.clone().some()
    }
}

pub fn has_type(r#type: Type) -> InferTypeRet { Quad::L(r#type) }
