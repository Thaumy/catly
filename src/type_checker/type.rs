use crate::parser::r#type::Type;

#[derive(Clone, Debug)]
pub enum TypeConstraint {
    // 已被绑定的确切类型
    Constraint(Type),
    // 等待约束的自由类型
    Free
}
