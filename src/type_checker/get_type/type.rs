use crate::infra::quad::Quad;
use crate::parser::r#type::Type;

// 顶层类型定义
pub type TypeEnv = Vec<(String, Type)>;

#[derive(Clone)]
pub enum TypeConstraint {
    // 已被绑定的确切类型
    Constraint(Type),
    // 等待约束的自由类型
    Free
}

pub type ExprEnv = Vec<(String, TypeConstraint)>;

// 经由约束才能使用的类型
#[derive(Clone)]
pub struct RequireConstraint {
    pub r#type: Type,
    pub constraint: Vec<(String, Type)>
}

// 需要类型信息
// 此情况由 env_ref case 产生时表明缺乏 ref_name 的类型信息
// discard case 产生该情况则表明某个弃元值缺乏关键的类型信息
#[derive(Clone)]
pub struct RequireInfo {
    pub ref_name: String
}

#[derive(Clone)]
pub struct TypeMissMatch {}

pub type GetTypeReturn =
    Quad<Type, RequireConstraint, RequireInfo, TypeMissMatch>;
