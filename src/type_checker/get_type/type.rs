use crate::infra::quad::Quad;
use crate::parser::r#type::Type;

// 经由约束才能使用的类型
#[derive(Clone, Debug)]
pub struct RequireConstraint {
    pub r#type: Type,
    pub constraint: Vec<(String, Type)>
}

// 需要类型信息
// 此情况由 env_ref case 产生时表明缺乏 ref_name 的类型信息
// discard case 产生该情况则表明某个弃元值缺乏关键的类型信息
#[derive(Clone, Debug)]
pub struct RequireInfo {
    pub ref_name: String
}

#[derive(Clone, Debug)]
pub struct TypeMissMatch {}

pub type GetTypeReturn =
    Quad<Type, RequireConstraint, RequireInfo, TypeMissMatch>;
