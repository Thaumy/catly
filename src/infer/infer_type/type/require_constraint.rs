use std::fmt::{Debug, Formatter};

use crate::infer::infer_type::r#fn::has_type;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::quad::QuadAnyExt;
use crate::parser::r#type::r#type::Type;

// 经由约束才能使用的类型
#[derive(PartialEq, Clone)]
pub struct RequireConstraint {
    pub r#type: Type,
    pub constraint: EnvRefConstraint
}

impl RequireConstraint {
    pub fn with_constraint_acc(
        &self,
        constraint: EnvRefConstraint
    ) -> InferTypeRet {
        // TODO: 考虑约束顺序对环境的影响
        require_extended_constraint(
            self.r#type.clone(),
            constraint,
            self.constraint.clone()
        )
    }
}

pub fn require_constraint(
    r#type: Type,
    constraint: EnvRefConstraint
) -> InferTypeRet {
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
) -> InferTypeRet {
    match l.extend_new(r.clone()) {
        Some(constraint) =>
            require_constraint(r#type, constraint.clone()),
        None => TypeMissMatch::of_constraint(&l, &r).into()
    }
}

impl From<RequireConstraint> for InferTypeRet {
    fn from(value: RequireConstraint) -> Self { value.quad_ml() }
}

impl Debug for RequireConstraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.constraint.is_empty() {
            f.write_str(&*format!("ReqConstraint::{:?}", self.r#type))
        } else {
            f.write_str(&*format!(
                "ReqConstraint::{:?} & {:?}",
                self.r#type, self.constraint
            ))
        }
    }
}
