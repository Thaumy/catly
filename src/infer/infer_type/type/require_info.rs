use std::fmt::{Debug, Formatter};

use crate::infer::infer_type::EnvRefConstraint;
use crate::infer::infer_type::InferTypeRet;
use crate::infer::infer_type::TypeMissMatch;
use crate::infra::WrapQuad;

// 需要类型信息
// 此情况由 namely case 产生时表明缺乏 ref_name 的类型信息
// discard case 产生该情况则表明某个弃元值缺乏关键的类型信息
#[derive(PartialEq, Clone)]
pub struct ReqInfo {
    pub ref_name: String,
    pub constraint: EnvRefConstraint
}

impl ReqInfo {
    pub fn of(
        ref_name: impl Into<String>,
        constraint: EnvRefConstraint
    ) -> ReqInfo {
        ReqInfo {
            ref_name: ref_name.into(),
            constraint
        }
    }

    pub fn new_ref_name(
        self,
        ref_name: impl Into<String>
    ) -> ReqInfo {
        ReqInfo {
            ref_name: ref_name.into(),
            constraint: self.constraint
        }
    }

    pub fn with_constraint_acc(
        self,
        acc: EnvRefConstraint
    ) -> InferTypeRet {
        match acc.extend_new(self.constraint.clone()) {
            Some(c) => ReqInfo::of(&self.ref_name, c).into(),
            None =>
                TypeMissMatch::of_constraint(&acc, &self.constraint)
                    .into(),
        }
    }
}

impl From<ReqInfo> for InferTypeRet {
    fn from(value: ReqInfo) -> Self { value.wrap_quad_mr() }
}

impl Debug for ReqInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.constraint.is_empty() {
            f.write_str(&format!("ReqInfo::{:?}", self.ref_name))
        } else {
            f.write_str(&format!(
                "ReqInfo::{:?} & {:?}",
                self.ref_name, self.constraint
            ))
        }
    }
}
