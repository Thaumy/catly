use std::fmt::{Debug, Formatter};

use crate::eval::r#type::r#type::MaybeType;
use crate::eval::r#type::r#type::Type;
use crate::infra::option::AnyExt;
use crate::infra::r#box::Ext as BoxAnyExt;
use crate::infra::vec::Ext as VecAnyExt;
use crate::parser::expr::r#type::Expr as CtExpr;
use crate::parser::r#type::r#type::Type as CtType;

pub type MaybeExpr = Option<Expr>;

pub type StructField = (String, Type, Expr);

#[derive(Clone, PartialEq)]
pub enum Expr {
    Unit(Type),
    Int(Type, i64),
    EnvRef(Type, String),
    Apply(Type, Box<Expr>, Box<Expr>),
    Cond(Type, Box<Expr>, Box<Expr>, Box<Expr>),
    Closure(Type, Option<String>, Type, Box<Expr>),
    Struct(Type, Vec<StructField>),
    Discard(Type),
    Match(Type, Box<Expr>, Vec<(Expr, Expr)>),
    Let(Type, String, Type, Box<Expr>, Box<Expr>)
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
            Expr::Apply(t, l, r) =>
                f.write_str(&*format!("(({l:?} {r:?}):{t:?})")),
            Expr::Cond(t, b, te, fe) => f.write_str(&*format!(
                "(if {b:?} then {te:?} else {fe:?}){}",
                type_annot(t)
            )),
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
            Expr::Match(t, t_e, vec) => f.write_str(&*format!(
                "(match {t_e:?} with {vec:?}){}",
                type_annot(t)
            )),
            Expr::Let(t, a_n, a_t, a_e, s_e) =>
                f.write_str(&*format!(
                    "(let {a_n}{} = {a_e:?} in {s_e:?}){}",
                    type_annot(a_t),
                    type_annot(t)
                )),
        }
    }
}

impl From<CtExpr> for MaybeExpr {
    fn from(value: CtExpr) -> Self {
        fn convert_type(t: CtType) -> MaybeType { t.into() }

        match value {
            CtExpr::Discard(Some(t)) =>
                Expr::Discard(convert_type(t)?),

            CtExpr::Unit(Some(t)) => Expr::Unit(convert_type(t)?),

            CtExpr::Int(Some(t), i) => Expr::Int(convert_type(t)?, i),

            CtExpr::EnvRef(Some(t), r_n) =>
                Expr::EnvRef(convert_type(t)?, r_n),

            CtExpr::Apply(Some(t), l_e, r_e) => Expr::Apply(
                convert_type(t)?,
                Self::from(*l_e)?.boxed(),
                Self::from(*r_e)?.boxed()
            ),

            CtExpr::Cond(Some(t), b_e, t_e, e_e) => Expr::Cond(
                convert_type(t)?,
                Self::from(*b_e)?.boxed(),
                Self::from(*t_e)?.boxed(),
                Self::from(*e_e)?.boxed()
            ),

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

            CtExpr::Match(Some(t), t_e, c_v) => {
                let t = convert_type(t)?;
                let t_e = Self::from(*t_e)?;

                c_v.iter()
                    .try_fold(vec![], |acc, (c_e, t_e)| {
                        let c_e = (c_e.clone().into(): MaybeExpr)?;
                        let t_e = (t_e.clone().into(): MaybeExpr)?;
                        acc.chain_push((c_e, t_e))
                            .some()
                    })
                    .map(|vec| Expr::Match(t, t_e.boxed(), vec))?
            }

            CtExpr::Let(Some(t), a_n, Some(a_t), a_e, o_e) =>
                Expr::Let(
                    convert_type(t)?,
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
