pub mod primitive_op;

use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::rc::Rc;

pub use primitive_op::*;

use crate::eval::env::ExprEnv;
use crate::eval::OptType;
use crate::eval::Type;
use crate::infra::VecExt;
use crate::infra::WrapOption;
use crate::infra::WrapRc;
use crate::parser::expr::r#type::Expr as CtExpr;
use crate::parser::r#type::Type as CtType;

pub type OptExpr = Option<Expr>;

pub type StructField = (String, Type, Rc<Expr>);

#[derive(Clone, PartialEq)]
pub enum Expr {
    Unit(Type),
    Int(Type, i64),
    EnvRef(Type, String),
    Closure(
        Type,
        Option<String>,
        Type,
        Rc<Expr>,
        Option<Rc<ExprEnv>>
    ),
    Struct(Type, Vec<StructField>),
    Discard(Type),

    PrimitiveOp(Type, Rc<PrimitiveOp>, Option<Rc<ExprEnv>>),

    Cond(Rc<Expr>, Rc<Expr>, Rc<Expr>),
    Match(Rc<Expr>, Vec<(Rc<Expr>, Rc<Expr>)>),
    Apply(Rc<Expr>, Rc<Expr>),
    Let(String, bool, Type, Rc<Expr>, Rc<Expr>)
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
        .wrap_some()
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn type_annot(t: &Type) -> String { format!(":{t:?}") }
        fn closure_input_name(s: &Option<String>) -> String {
            match s {
                Some(s) => format!("{s}"),
                None => "_".to_string()
            }
        }
        match self {
            Expr::Unit(t) =>
                f.write_str(&format!("(){}", type_annot(t))),
            Expr::Int(t, i) =>
                f.write_str(&format!("{i}{}", type_annot(t))),
            Expr::EnvRef(t, n) =>
                f.write_str(&format!("{n}{}", type_annot(t))),
            Expr::Closure(t, i_n, i_t, o_e, env) =>
                f.write_str(&format!(
                    "({}{} ->[env:{env:p}] {o_e:?}){}",
                    closure_input_name(i_n),
                    type_annot(i_t),
                    type_annot(t)
                )),
            Expr::Struct(t, vec) => f.write_str(&format!(
                "{{ {vec:?}{} }}",
                type_annot(t)
            )),
            Expr::Discard(t) =>
                f.write_str(&format!("_{}", type_annot(t))),

            Expr::PrimitiveOp(t, op, l_env) => f.write_str(&format!(
                "([l_env:{l_env:p}]{op:?}){}",
                type_annot(t)
            )),

            Expr::Cond(b, te, fe) => f.write_str(&format!(
                "if {b:?} then {te:?} else {fe:?}",
            )),
            Expr::Match(t_e, vec) =>
                f.write_str(&format!("match {t_e:?} with {vec:?}",)),
            Expr::Apply(l, r) =>
                f.write_str(&format!("({l:?} {r:?})")),
            Expr::Let(a_n, r_a, a_t, a_e, s_e) => {
                let r_a = if *r_a { "rec " } else { "" };
                f.write_str(&format!(
                    "let {r_a}{a_n}{} = {a_e:?} in {s_e:?}",
                    type_annot(a_t),
                ))
            }
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
                    Some(op) =>
                        Expr::PrimitiveOp(t, op.wrap_rc(), None),
                    None => Expr::EnvRef(t, r_n)
                }
            }

            CtExpr::Closure(Some(t), i_n, Some(i_t), o_e) =>
                Expr::Closure(
                    convert_type(t)?,
                    i_n,
                    convert_type(i_t)?,
                    Self::from(o_e.deref().clone())?.wrap_rc(),
                    None
                ),

            CtExpr::Struct(Some(t), s_v) => {
                let t = convert_type(t)?;

                s_v.into_iter()
                    .try_fold(vec![], |acc, (sf_n, sf_t, sf_e)| {
                        let sf_t = convert_type(sf_t?)?;
                        let sf_e = Self::from(sf_e)?;
                        acc.chain_push((sf_n, sf_t, sf_e.wrap_rc()))
                            .wrap_some()
                    })
                    .map(|vec| Expr::Struct(t, vec))?
            }

            CtExpr::Cond(Some(_), b_e, t_e, e_e) => Expr::Cond(
                Self::from(b_e.deref().clone())?.wrap_rc(),
                Self::from(t_e.deref().clone())?.wrap_rc(),
                Self::from(e_e.deref().clone())?.wrap_rc()
            ),

            CtExpr::Match(Some(_), t_e, c_v) => {
                let t_e = Self::from(t_e.deref().clone())?;

                c_v.into_iter()
                    .try_fold(vec![], |acc, (c_e, t_e)| {
                        let c_e = OptExpr::from(c_e)?;
                        let t_e = OptExpr::from(t_e)?;
                        acc.chain_push((c_e.wrap_rc(), t_e.wrap_rc()))
                            .wrap_some()
                    })
                    .map(|vec| Expr::Match(t_e.wrap_rc(), vec))?
            }

            CtExpr::Apply(Some(_), l_e, r_e) => Expr::Apply(
                Self::from(l_e.deref().clone())?.wrap_rc(),
                Self::from(r_e.deref().clone())?.wrap_rc()
            ),

            CtExpr::Let(Some(_), r_a, a_n, Some(a_t), a_e, o_e) =>
                Expr::Let(
                    a_n,
                    r_a,
                    convert_type(a_t)?,
                    Self::from(a_e.deref().clone())?.wrap_rc(),
                    Self::from(o_e.deref().clone())?.wrap_rc()
                ),
            _ => return None
        }
        .wrap_some()
    }
}
