use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct EvalErr {
    pub info: String
}

impl EvalErr {
    pub fn of(info: impl Into<String>) -> EvalErr {
        EvalErr { info: info.into() }
    }
}

impl Debug for EvalErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("EvalErr::{:?}", self.info))
    }
}
