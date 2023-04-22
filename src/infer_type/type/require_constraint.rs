use std::fmt::{Debug, Formatter};

use crate::infer_type::r#fn::has_type;
use crate::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infer_type::r#type::GetTypeReturn;
use crate::infra::quad::Quad;
use crate::parser::r#type::r#type::Type;

// 经由约束才能使用的类型
#[derive(PartialEq, Clone)]
pub struct RequireConstraint {
    pub r#type: Type,
    pub constraint: EnvRefConstraint
}

pub fn require_constraint(
    r#type: Type,
    constraint: EnvRefConstraint
) -> GetTypeReturn {
    if constraint.is_empty() {
        has_type(r#type)
    } else {
        RequireConstraint { r#type, constraint }.into()
    }
}

pub fn require_extended_constraint(
    r#type: Type,
    l: EnvRefConstraint,
    r: EnvRefConstraint
) -> GetTypeReturn {
    match l.extend_new(r.clone()) {
        Some(constraint) =>
            require_constraint(r#type, constraint.clone()),
        None => TypeMissMatch::of_constraint(&l, &r).into()
    }
}

impl From<RequireConstraint> for GetTypeReturn {
    fn from(value: RequireConstraint) -> Self { Quad::ML(value) }
}

impl Debug for RequireConstraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!(
            "ReqConstraint::{:?} & {:?}",
            self.r#type, self.constraint
        ))
    }
}
