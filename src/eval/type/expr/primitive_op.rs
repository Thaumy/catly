use crate::eval::r#macro::bool_type;
use crate::eval::r#macro::closure_type;
use crate::eval::r#macro::int_type;
use crate::eval::r#type::expr::Expr;
use crate::infra::r#box::Ext;

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveOp {
    Neg,
    Add(Option<Expr>),
    Sub(Option<Expr>),
    Mul(Option<Expr>),
    Mod(Option<Expr>),
    Rem(Option<Expr>),

    Gt(Option<Expr>),
    Eq(Option<Expr>),
    Lt(Option<Expr>),

    Not,
    And(Option<Expr>),
    Or(Option<Expr>)
}

impl From<PrimitiveOp> for Expr {
    fn from(value: PrimitiveOp) -> Self {
        match value {
            // neg
            PrimitiveOp::Neg => Expr::PrimitiveOp(
                closure_type!(int_type!(), int_type!()),
                value.boxed()
            ),
            // add
            PrimitiveOp::Add(None) => Expr::PrimitiveOp(
                closure_type!(
                    int_type!(),
                    closure_type!(int_type!(), int_type!())
                ),
                value.boxed()
            ),
            PrimitiveOp::Add(Some(_)) => Expr::PrimitiveOp(
                closure_type!(int_type!(), int_type!()),
                value.boxed()
            ),
            // sub
            PrimitiveOp::Sub(None) => Expr::PrimitiveOp(
                closure_type!(
                    int_type!(),
                    closure_type!(int_type!(), int_type!())
                ),
                value.boxed()
            ),
            PrimitiveOp::Sub(Some(_)) => Expr::PrimitiveOp(
                closure_type!(int_type!(), int_type!()),
                value.boxed()
            ),
            // mul
            PrimitiveOp::Mul(None) => Expr::PrimitiveOp(
                closure_type!(
                    int_type!(),
                    closure_type!(int_type!(), int_type!())
                ),
                value.boxed()
            ),
            PrimitiveOp::Mul(Some(_)) => Expr::PrimitiveOp(
                closure_type!(int_type!(), int_type!()),
                value.boxed()
            ),
            // mod
            PrimitiveOp::Mod(None) => Expr::PrimitiveOp(
                closure_type!(
                    int_type!(),
                    closure_type!(int_type!(), int_type!())
                ),
                value.boxed()
            ),
            PrimitiveOp::Mod(Some(_)) => Expr::PrimitiveOp(
                closure_type!(int_type!(), int_type!()),
                value.boxed()
            ),
            // rem
            PrimitiveOp::Rem(None) => Expr::PrimitiveOp(
                closure_type!(
                    int_type!(),
                    closure_type!(int_type!(), int_type!())
                ),
                value.boxed()
            ),
            PrimitiveOp::Rem(Some(_)) => Expr::PrimitiveOp(
                closure_type!(int_type!(), int_type!()),
                value.boxed()
            ),

            // gt
            PrimitiveOp::Gt(None) => Expr::PrimitiveOp(
                closure_type!(
                    int_type!(),
                    closure_type!(int_type!(), bool_type!())
                ),
                value.boxed()
            ),
            PrimitiveOp::Gt(Some(_)) => Expr::PrimitiveOp(
                closure_type!(int_type!(), bool_type!()),
                value.boxed()
            ),
            // eq
            PrimitiveOp::Eq(None) => Expr::PrimitiveOp(
                closure_type!(
                    int_type!(),
                    closure_type!(int_type!(), bool_type!())
                ),
                value.boxed()
            ),
            PrimitiveOp::Eq(Some(_)) => Expr::PrimitiveOp(
                closure_type!(int_type!(), bool_type!()),
                value.boxed()
            ),
            // lt
            PrimitiveOp::Lt(None) => Expr::PrimitiveOp(
                closure_type!(
                    int_type!(),
                    closure_type!(int_type!(), bool_type!())
                ),
                value.boxed()
            ),
            PrimitiveOp::Lt(Some(_)) => Expr::PrimitiveOp(
                closure_type!(int_type!(), bool_type!()),
                value.boxed()
            ),

            // not
            PrimitiveOp::Not => Expr::PrimitiveOp(
                closure_type!(bool_type!(), bool_type!()),
                value.boxed()
            ),
            // and
            PrimitiveOp::And(None) => Expr::PrimitiveOp(
                closure_type!(
                    bool_type!(),
                    closure_type!(bool_type!(), bool_type!())
                ),
                value.boxed()
            ),
            PrimitiveOp::And(Some(_)) => Expr::PrimitiveOp(
                closure_type!(bool_type!(), bool_type!()),
                value.boxed()
            ),
            // or
            PrimitiveOp::Or(None) => Expr::PrimitiveOp(
                closure_type!(
                    bool_type!(),
                    closure_type!(bool_type!(), bool_type!())
                ),
                value.boxed()
            ),
            PrimitiveOp::Or(Some(_)) => Expr::PrimitiveOp(
                closure_type!(bool_type!(), bool_type!()),
                value.boxed()
            )
        }
    }
}
