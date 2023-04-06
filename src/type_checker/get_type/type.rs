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

#[derive(Clone)]
pub struct RequireInfo {}

#[derive(Clone)]
pub struct TypeMissMatch {}

pub type GetTypeReturn =
    Quad<Type, RequireConstraint, RequireInfo, TypeMissMatch>;
