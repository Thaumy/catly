pub mod env_ref_constraint;

use std::fmt::{Debug, Formatter};

use crate::get_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::parser::r#type::r#type::Type;

// 经由约束才能使用的类型
#[derive(PartialEq, Clone)]
pub struct RequireConstraint {
    pub r#type: Type,
    pub constraint: EnvRefConstraint
}

impl Debug for RequireConstraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!(
            "ReqConstraint::{:?} & {:?}",
            self.r#type, self.constraint
        ))
    }
}

// 需要类型信息
// 此情况由 namely case 产生时表明缺乏 ref_name 的类型信息
// discard case 产生该情况则表明某个弃元值缺乏关键的类型信息
#[derive(PartialEq, Clone)]
pub struct RequireInfo {
    pub ref_name: String
}

impl Debug for RequireInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("ReqInfo::{:?}", self.ref_name))
    }
}

#[derive(PartialEq, Clone)]
pub struct TypeMissMatch {
    pub info: String
}

impl Debug for TypeMissMatch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("TypeMissMatch::{:?}", self.info))
    }
}

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
