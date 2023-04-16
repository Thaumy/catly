use crate::infra::alias::MaybeExpr;
use crate::parser::expr::r#type::Expr;

// 表示 EnvRef 的源表达式
#[derive(Clone, Debug)]
pub enum EnvRefSrc {
    Src(Expr),
    NoSrc
}

impl From<Expr> for EnvRefSrc {
    fn from(value: Expr) -> Self { EnvRefSrc::Src(value) }
}

impl From<MaybeExpr> for EnvRefSrc {
    fn from(value: MaybeExpr) -> Self {
        match value {
            Some(e) => EnvRefSrc::Src(e),
            None => EnvRefSrc::NoSrc
        }
    }
}

impl From<EnvRefSrc> for MaybeExpr {
    fn from(value: EnvRefSrc) -> Self {
        match value {
            EnvRefSrc::Src(e) => Some(e),
            EnvRefSrc::NoSrc => None
        }
    }
}
