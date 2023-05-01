use std::fmt::{Debug, Formatter};

use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infra::quad::AnyExt;
use crate::parser::r#type::r#type::Type;

#[derive(PartialEq, Clone)]
pub struct TypeMissMatch {
    pub info: String
}

impl TypeMissMatch {
    pub fn of(info: impl Into<String>) -> TypeMissMatch {
        TypeMissMatch { info: info.into() }
    }

    pub fn of_type(l: &Type, r: &Type) -> TypeMissMatch {
        TypeMissMatch {
            info: format!("{l:?} <> {r:?}")
        }
    }

    pub fn of_constraint(
        l: &EnvRefConstraint,
        r: &EnvRefConstraint
    ) -> TypeMissMatch {
        TypeMissMatch {
            info: format!("Constraint conflict: {l:?} <> {r:?}",)
        }
    }

    pub fn of_dup_capture(
        old: impl Debug,
        new: impl Debug
    ) -> TypeMissMatch {
        TypeMissMatch::of(format!(
            "Duplicate capture in case pattern: {old:?} <old/new> {new:?}"
        ))
    }
}

impl From<TypeMissMatch> for InferTypeRet {
    fn from(value: TypeMissMatch) -> Self { value.quad_r() }
}

impl Debug for TypeMissMatch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("TypeMissMatch::{:?}", self.info))
    }
}
