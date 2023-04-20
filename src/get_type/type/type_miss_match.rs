use std::fmt::{Debug, Formatter};

#[derive(PartialEq, Clone)]
pub struct TypeMissMatch {
    pub info: String
}

impl Debug for TypeMissMatch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("TypeMissMatch::{:?}", self.info))
    }
}
