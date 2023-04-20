use std::fmt::{Debug, Formatter};

use crate::get_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::parser::r#type::r#type::Type;

// 经由约束才能使用的类型
#[derive(PartialEq, Clone)]
pub struct RequireConstraint {
    pub r#type: Type,
    pub constraint: EnvRefConstraint
}

impl Debug for RequireConstraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!(
            "ReqConstraint::{:?} & {:?}",
            self.r#type, self.constraint
        ))
    }
}
