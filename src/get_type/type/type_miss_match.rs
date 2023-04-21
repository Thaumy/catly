use std::fmt::{Debug, Formatter};

use crate::get_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::get_type::r#type::GetTypeReturn;
use crate::infra::quad::Quad;
use crate::parser::r#type::r#type::Type;

#[derive(PartialEq, Clone)]
pub struct TypeMissMatch {
    pub info: String
}

impl TypeMissMatch {
    pub fn of(info: &str) -> TypeMissMatch {
        TypeMissMatch {
            info: info.to_string()
        }
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
}

impl From<TypeMissMatch> for GetTypeReturn {
    fn from(value: TypeMissMatch) -> Self { Quad::R(value) }
}

impl Debug for TypeMissMatch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("TypeMissMatch::{:?}", self.info))
    }
}
