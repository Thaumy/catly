type In = crate::parser::preprocess::Out;

pub trait FollowExt<T> {
    fn is_expr_end_pat(&self) -> bool;
    fn is_type_end_pat(&self) -> bool;
}

impl FollowExt<In> for Option<In> {
    fn is_expr_end_pat(&self) -> bool {
        match self {
            None |
            Some(In::Symbol(')')) |// ..
            Some(In::Symbol('}')) |// Struct
            Some(In::Symbol(',')) |// Assign (Struct, Let
            Some(In::Symbol('|')) |// Match
            Some(In::Symbol('=')) |// Assign (Struct, Let
            Some(In::Kw(_))// 这意味着`最近可立即归约`的语言构造具备更高的结合优先级
            => true,
            _ => false,
        }
    }
    fn is_type_end_pat(&self) -> bool {
        match self {
            None |
            Some(In::Symbol(')')) |
            Some(In::Symbol('}')) |
            Some(In::Symbol(',')) |
            Some(In::Symbol('=')) |
            Some(In::Kw(_)) |
            Some(In::LetName(_)) |
            Some(In::TypeName(_)) |
            Some(In::IntValue(_)) |
            Some(In::UnitValue) |
            Some(In::DiscardValue) => true,

            _ => false
        }
    }
}

pub trait AnyExt
where
    Self: Sized
{
    fn some(self) -> Option<Self> { Some(self) }
}

impl<T> AnyExt for T {}
