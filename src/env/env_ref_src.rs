use crate::infra::alias::MaybeExpr;
use crate::parser::expr::Expr;

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
