use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infra::option::OptionAnyExt;
use crate::infra::quad::QuadAnyExt;
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
            let base_type = type_env.find_type(type_name.as_str())?;
            destruct_namely_type(type_env, &base_type)
        }
        x => x.clone().some()
    }
}

pub fn has_type(r#type: Type) -> InferTypeRet { r#type.quad_l() }
