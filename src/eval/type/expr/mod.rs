pub mod primitive_op;

use std::fmt::{Debug, Formatter};

use crate::eval::r#type::expr::primitive_op::PrimitiveOp;
use crate::eval::r#type::r#type::OptType;
use crate::eval::r#type::r#type::Type;
use crate::infra::option::OptionAnyExt;
use crate::infra::r#box::BoxAnyExt;
use crate::infra::vec::VecExt;
use crate::parser::expr::r#type::Expr as CtExpr;
use crate::parser::r#type::r#type::Type as CtType;

pub type OptExpr = Option<Expr>;

pub type StructField = (String, Type, Expr);

#[derive(Clone, PartialEq)]
pub enum Expr {
    Unit(Type),
    Int(Type, i64),
    EnvRef(Type, String),
    Closure(Type, Option<String>, Type, Box<Expr>),
    Struct(Type, Vec<StructField>),
    Discard(Type),

    PrimitiveOp(Type, Box<PrimitiveOp>),

    Cond(Box<Expr>, Box<Expr>, Box<Expr>),
    Match(Box<Expr>, Vec<(Expr, Expr)>),
    Apply(Box<Expr>, Box<Expr>),
    Let(String, Type, Box<Expr>, Box<Expr>)
}

impl Expr {
    pub fn get_type_annot(&self) -> OptType {
        match self {
            Expr::Unit(t) => t,
            Expr::Int(t, ..) => t,
            Expr::EnvRef(t, ..) => t,
            Expr::Closure(t, ..) => t,
            Expr::Struct(t, ..) => t,
            Expr::Discard(t, ..) => t,

            Expr::PrimitiveOp(t, ..) => t,

            _ => return None
        }
        .clone()
        .some()
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn type_annot(t: &Type) -> String { format!(":{t:?}") }
        fn closure_input_name(s: &Option<String>) -> String {
            match s {
                Some(s) => format!("{s}"),
                None => format!("_")
            }
        }
        match self {
            Expr::Unit(t) =>
                f.write_str(&*format!("(){}", type_annot(t))),
            Expr::Int(t, i) =>
                f.write_str(&*format!("{i}{}", type_annot(t))),
            Expr::EnvRef(t, n) =>
                f.write_str(&*format!("{n}{}", type_annot(t))),
            Expr::Closure(t, i_n, i_t, o_e) =>
                f.write_str(&*format!(
                    "({}{} -> {o_e:?}){}",
                    closure_input_name(i_n),
                    type_annot(i_t),
                    type_annot(t)
                )),
            Expr::Struct(t, vec) => f.write_str(&*format!(
                "{{ {vec:?}{} }}",
                type_annot(t)
            )),
            Expr::Discard(t) =>
                f.write_str(&*format!("_{}", type_annot(t))),

            Expr::PrimitiveOp(t, op) =>
                f.write_str(&*format!("({op:?}){}", type_annot(t))),

            Expr::Cond(b, te, fe) => f.write_str(&*format!(
                "if {b:?} then {te:?} else {fe:?}",
            )),
            Expr::Match(t_e, vec) =>
                f.write_str(&*format!("match {t_e:?} with {vec:?}",)),
            Expr::Apply(l, r) =>
                f.write_str(&*format!("({l:?} {r:?})")),
            Expr::Let(a_n, a_t, a_e, s_e) => f.write_str(&*format!(
                "let {a_n}{} = {a_e:?} in {s_e:?}",
                type_annot(a_t),
            ))
        }
    }
}

impl From<CtExpr> for OptExpr {
    fn from(value: CtExpr) -> Self {
        fn convert_type(t: CtType) -> OptType { t.into() }

        match value {
            CtExpr::Discard(Some(t)) =>
                Expr::Discard(convert_type(t)?),

            CtExpr::Unit(Some(t)) => Expr::Unit(convert_type(t)?),

            CtExpr::Int(Some(t), i) => Expr::Int(convert_type(t)?, i),

            CtExpr::EnvRef(Some(t), r_n) => {
                let t = convert_type(t)?;

                match PrimitiveOp::from_env_ref(r_n.clone().as_str())
                {
                    Some(op) => Expr::PrimitiveOp(t, op.boxed()),
                    None => Expr::EnvRef(t, r_n)
                }
            }

            CtExpr::Closure(Some(t), i_n, Some(i_t), o_e) =>
                Expr::Closure(
                    convert_type(t)?,
                    i_n,
                    convert_type(i_t)?,
                    Self::from(*o_e)?.boxed()
                ),

            CtExpr::Struct(Some(t), s_v) => {
                let t = convert_type(t)?;

                s_v.iter()
                    .try_fold(vec![], |acc, (sf_n, sf_t, sf_e)| {
                        let sf_n = sf_n.to_string();
                        let sf_t = convert_type(sf_t.clone()?)?;
                        let sf_e = Self::from(sf_e.clone())?;
                        acc.chain_push((sf_n, sf_t, sf_e))
                            .some()
                    })
                    .map(|vec| Expr::Struct(t, vec))?
            }

            CtExpr::Cond(Some(_), b_e, t_e, e_e) => Expr::Cond(
                Self::from(*b_e)?.boxed(),
                Self::from(*t_e)?.boxed(),
                Self::from(*e_e)?.boxed()
            ),

            CtExpr::Match(Some(_), t_e, c_v) => {
                let t_e = Self::from(*t_e)?;

                c_v.iter()
                    .try_fold(vec![], |acc, (c_e, t_e)| {
                        let c_e = (c_e.clone().into(): OptExpr)?;
                        let t_e = (t_e.clone().into(): OptExpr)?;
                        acc.chain_push((c_e, t_e))
                            .some()
                    })
                    .map(|vec| Expr::Match(t_e.boxed(), vec))?
            }

            CtExpr::Apply(Some(_), l_e, r_e) => Expr::Apply(
                Self::from(*l_e)?.boxed(),
                Self::from(*r_e)?.boxed()
            ),

            CtExpr::Let(Some(_), a_n, Some(a_t), a_e, o_e) =>
                Expr::Let(
                    a_n,
                    convert_type(a_t)?,
                    Self::from(*a_e)?.boxed(),
                    Self::from(*o_e)?.boxed()
                ),
            _ => return None
        }
        .some()
    }
}
