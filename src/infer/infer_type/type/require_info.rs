use std::fmt::{Debug, Formatter};

use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::quad::QuadAnyExt;

// 需要类型信息
// 此情况由 namely case 产生时表明缺乏 ref_name 的类型信息
// discard case 产生该情况则表明某个弃元值缺乏关键的类型信息
#[derive(PartialEq, Clone)]
pub struct RequireInfo {
    pub ref_name: String,
    pub constraint: EnvRefConstraint
}

impl RequireInfo {
    pub fn of<'s>(
        ref_name: impl Into<String>,
        constraint: EnvRefConstraint
    ) -> RequireInfo {
        RequireInfo {
            ref_name: ref_name.into(),
            constraint
        }
    }

    pub fn new_ref_name(
        self,
        ref_name: impl Into<String>
    ) -> RequireInfo {
        RequireInfo {
            ref_name: ref_name.into(),
            constraint: self.constraint
        }
    }

    pub fn with_constraint_acc(
        self,
        constraint: EnvRefConstraint
    ) -> InferTypeRet {
        match self
            .constraint
            .extend_new(constraint.clone())
        {
            Some(constraint) =>
                RequireInfo::of(&self.ref_name, constraint).into(),
            None => TypeMissMatch::of_constraint(
                &self.constraint,
                &constraint
            )
            .into()
        }
    }
}

impl From<RequireInfo> for InferTypeRet {
    fn from(value: RequireInfo) -> Self { value.quad_mr() }
}

impl Debug for RequireInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.constraint.is_empty() {
            f.write_str(&*format!("ReqInfo::{:?}", self.ref_name))
        } else {
            f.write_str(&*format!(
                "ReqInfo::{:?} & {:?}",
                self.ref_name, self.constraint
            ))
        }
    }
}
