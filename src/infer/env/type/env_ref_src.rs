use crate::parser::expr::r#type::{Expr, OptExpr};

// 表示 EnvRef 的源表达式
#[derive(Clone, Debug)]
pub enum EnvRefSrc {
    Src(Expr),
    NoSrc
}

impl From<Expr> for EnvRefSrc {
    fn from(value: Expr) -> Self { EnvRefSrc::Src(value) }
}

impl From<OptExpr> for EnvRefSrc {
    fn from(value: OptExpr) -> Self {
        match value {
            Some(e) => EnvRefSrc::Src(e),
            None => EnvRefSrc::NoSrc
        }
    }
}

impl From<EnvRefSrc> for OptExpr {
    fn from(value: EnvRefSrc) -> Self {
        match value {
            EnvRefSrc::Src(e) => Some(e),
            EnvRefSrc::NoSrc => None
        }
    }
}
