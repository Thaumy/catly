use crate::parser::r#type::Type;

pub fn lift(derive: &Type) -> bool {
    println!("Uplift {:?} to {:?}", "Discard", derive);

    true
}
