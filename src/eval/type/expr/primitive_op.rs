use crate::eval::r#macro::bool_type;
use crate::eval::r#macro::closure_type;
use crate::eval::r#macro::int_type;
use crate::eval::r#type::expr::Expr;
use crate::infra::option::OptionAnyExt;
use crate::infra::r#box::BoxAnyExt;

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveOp {
    Neg,
    Add(Option<Expr>),
    Sub(Option<Expr>),
    Mul(Option<Expr>),
    Div(Option<Expr>),
    Mod(Option<Expr>),
    Rem(Option<Expr>),

    Gt(Option<Expr>),
    Eq(Option<Expr>),
    Lt(Option<Expr>),

    Not,
    And(Option<Expr>),
    Or(Option<Expr>)
}

impl PrimitiveOp {
    pub fn from_env_ref<'s>(
        ref_name: impl Into<&'s str>
    ) -> Option<PrimitiveOp> {
        match ref_name.into() {
            "neg" => PrimitiveOp::Neg,
            "add" => PrimitiveOp::Add(None),
            "sub" => PrimitiveOp::Sub(None),
            "mul" => PrimitiveOp::Mul(None),
            "div" => PrimitiveOp::Div(None),
            "mod" => PrimitiveOp::Mod(None),
            "rem" => PrimitiveOp::Rem(None),

            "gt" => PrimitiveOp::Gt(None),
            "eq" => PrimitiveOp::Eq(None),
            "lt" => PrimitiveOp::Lt(None),

            "not" => PrimitiveOp::Not,
            "and" => PrimitiveOp::And(None),
            "or" => PrimitiveOp::Or(None),

            _ => return None
        }
        .some()
    }
}

impl From<PrimitiveOp> for Expr {
    fn from(value: PrimitiveOp) -> Self {
        match value {
            // neg
            PrimitiveOp::Neg => Expr::PrimitiveOp(
                closure_type!(int_type!(), int_type!()),
                value.boxed(),
                None
            ),
            // add
            PrimitiveOp::Add(None) => Expr::PrimitiveOp(
                closure_type!(
                    int_type!(),
                    closure_type!(int_type!(), int_type!())
                ),
                value.boxed(),
                None
            ),
            PrimitiveOp::Add(Some(_)) => Expr::PrimitiveOp(
                closure_type!(int_type!(), int_type!()),
                value.boxed(),
                None
            ),
            // sub
            PrimitiveOp::Sub(None) => Expr::PrimitiveOp(
                closure_type!(
                    int_type!(),
                    closure_type!(int_type!(), int_type!())
                ),
                value.boxed(),
                None
            ),
            PrimitiveOp::Sub(Some(_)) => Expr::PrimitiveOp(
                closure_type!(int_type!(), int_type!()),
                value.boxed(),
                None
            ),
            // mul
            PrimitiveOp::Mul(None) => Expr::PrimitiveOp(
                closure_type!(
                    int_type!(),
                    closure_type!(int_type!(), int_type!())
                ),
                value.boxed(),
                None
            ),
            PrimitiveOp::Mul(Some(_)) => Expr::PrimitiveOp(
                closure_type!(int_type!(), int_type!()),
                value.boxed(),
                None
            ),
            // div
            PrimitiveOp::Div(None) => Expr::PrimitiveOp(
                closure_type!(
                    int_type!(),
                    closure_type!(int_type!(), int_type!())
                ),
                value.boxed(),
                None
            ),
            PrimitiveOp::Div(Some(_)) => Expr::PrimitiveOp(
                closure_type!(int_type!(), int_type!()),
                value.boxed(),
                None
            ),
            // mod
            PrimitiveOp::Mod(None) => Expr::PrimitiveOp(
                closure_type!(
                    int_type!(),
                    closure_type!(int_type!(), int_type!())
                ),
                value.boxed(),
                None
            ),
            PrimitiveOp::Mod(Some(_)) => Expr::PrimitiveOp(
                closure_type!(int_type!(), int_type!()),
                value.boxed(),
                None
            ),
            // rem
            PrimitiveOp::Rem(None) => Expr::PrimitiveOp(
                closure_type!(
                    int_type!(),
                    closure_type!(int_type!(), int_type!())
                ),
                value.boxed(),
                None
            ),
            PrimitiveOp::Rem(Some(_)) => Expr::PrimitiveOp(
                closure_type!(int_type!(), int_type!()),
                value.boxed(),
                None
            ),

            // gt
            PrimitiveOp::Gt(None) => Expr::PrimitiveOp(
                closure_type!(
                    int_type!(),
                    closure_type!(int_type!(), bool_type!())
                ),
                value.boxed(),
                None
            ),
            PrimitiveOp::Gt(Some(_)) => Expr::PrimitiveOp(
                closure_type!(int_type!(), bool_type!()),
                value.boxed(),
                None
            ),
            // eq
            PrimitiveOp::Eq(None) => Expr::PrimitiveOp(
                closure_type!(
                    int_type!(),
                    closure_type!(int_type!(), bool_type!())
                ),
                value.boxed(),
                None
            ),
            PrimitiveOp::Eq(Some(_)) => Expr::PrimitiveOp(
                closure_type!(int_type!(), bool_type!()),
                value.boxed(),
                None
            ),
            // lt
            PrimitiveOp::Lt(None) => Expr::PrimitiveOp(
                closure_type!(
                    int_type!(),
                    closure_type!(int_type!(), bool_type!())
                ),
                value.boxed(),
                None
            ),
            PrimitiveOp::Lt(Some(_)) => Expr::PrimitiveOp(
                closure_type!(int_type!(), bool_type!()),
                value.boxed(),
                None
            ),

            // not
            PrimitiveOp::Not => Expr::PrimitiveOp(
                closure_type!(bool_type!(), bool_type!()),
                value.boxed(),
                None
            ),
            // and
            PrimitiveOp::And(None) => Expr::PrimitiveOp(
                closure_type!(
                    bool_type!(),
                    closure_type!(bool_type!(), bool_type!())
                ),
                value.boxed(),
                None
            ),
            PrimitiveOp::And(Some(_)) => Expr::PrimitiveOp(
                closure_type!(bool_type!(), bool_type!()),
                value.boxed(),
                None
            ),
            // or
            PrimitiveOp::Or(None) => Expr::PrimitiveOp(
                closure_type!(
                    bool_type!(),
                    closure_type!(bool_type!(), bool_type!())
                ),
                value.boxed(),
                None
            ),
            PrimitiveOp::Or(Some(_)) => Expr::PrimitiveOp(
                closure_type!(bool_type!(), bool_type!()),
                value.boxed(),
                None
            )
        }
    }
}
