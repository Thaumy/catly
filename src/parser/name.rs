use crate::parser::name::let_name::parse_let_name;
use crate::parser::name::type_name::parse_type_name;

pub mod let_name;
pub mod type_name;

pub enum Name {
    LetName(String),
    TypeName(String),
}

pub fn parse_name(str: &str) -> Option<Name> {
    if str.starts_with(|c: char| c.is_uppercase()) {
        match parse_type_name(str) {
            Some(n) => Some(Name::TypeName(n.to_string())),
            _ => None,
        }
    } else {
        match parse_let_name(str) {
            // _ -> LetName
            Some(n) => Some(Name::LetName(n.to_string())),
            _ => None
        }
    }
}