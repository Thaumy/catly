use std::fmt::{Debug, Formatter};

use crate::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infer_type::r#type::GetTypeReturn;
use crate::infra::quad::Quad;

// 需要类型信息
// 此情况由 namely case 产生时表明缺乏 ref_name 的类型信息
// discard case 产生该情况则表明某个弃元值缺乏关键的类型信息
#[derive(PartialEq, Clone)]
pub struct RequireInfo {
    pub ref_name: String,
    pub constraint: EnvRefConstraint
}

impl RequireInfo {
    pub fn of(
        ref_name: &str,
        constraint: EnvRefConstraint
    ) -> RequireInfo {
        RequireInfo {
            ref_name: ref_name.to_string(),
            constraint
        }
    }

    pub fn new_ref_name(self, ref_name: &str) -> RequireInfo {
        RequireInfo {
            ref_name: ref_name.to_string(),
            constraint: self.constraint
        }
    }

    pub fn with_constraint_acc(
        self,
        constraint: EnvRefConstraint
    ) -> GetTypeReturn {
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

impl From<RequireInfo> for GetTypeReturn {
    fn from(value: RequireInfo) -> Self { Quad::MR(value) }
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
