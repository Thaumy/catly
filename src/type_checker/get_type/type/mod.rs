pub mod env_ref_constraint;

use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::parser::r#type::r#type::Type;
use crate::type_checker::get_type::r#type::env_ref_constraint::EnvRefConstraint;

// 经由约束才能使用的类型
#[derive(PartialEq, Clone, Debug)]
pub struct RequireConstraint {
    pub r#type: Type,
    pub constraint: EnvRefConstraint
}

// 需要类型信息
// 此情况由 namely case 产生时表明缺乏 ref_name 的类型信息
// discard case 产生该情况则表明某个弃元值缺乏关键的类型信息
#[derive(PartialEq, Clone, Debug)]
pub struct RequireInfo {
    pub ref_name: String
}

#[derive(PartialEq, Clone, Debug)]
pub struct TypeMissMatch {}

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
