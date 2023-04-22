pub mod env_ref_constraint;
pub mod require_constraint;
pub mod require_info;
pub mod type_miss_match;

use crate::infer_type::r#type::require_constraint::RequireConstraint;
use crate::infer_type::r#type::require_info::RequireInfo;
use crate::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::parser::r#type::r#type::Type;

pub type GetTypeReturn =
    Quad<Type, RequireConstraint, RequireInfo, TypeMissMatch>;

impl From<GetTypeReturn> for MaybeType {
    fn from(value: GetTypeReturn) -> Self {
        match value {
            Quad::L(t) => t.some(),
            _ => None
        }
    }
}
