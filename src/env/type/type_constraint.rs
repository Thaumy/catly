use crate::parser::r#type::r#type::MaybeType;
use crate::parser::r#type::r#type::Type;

#[derive(Clone, Debug)]
pub enum TypeConstraint {
    // 已被绑定的确切类型
    Constraint(Type),
    // 等待约束的自由类型
    Free
}

impl From<Type> for TypeConstraint {
    fn from(value: Type) -> Self { TypeConstraint::Constraint(value) }
}

impl From<MaybeType> for TypeConstraint {
    fn from(value: MaybeType) -> Self {
        match value {
            Some(t) => TypeConstraint::Constraint(t),
            None => TypeConstraint::Free
        }
    }
}

impl From<TypeConstraint> for MaybeType {
    fn from(value: TypeConstraint) -> Self {
        match value {
            TypeConstraint::Constraint(t) => Some(t),
            TypeConstraint::Free => None
        }
    }
}
