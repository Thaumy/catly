use std::fmt::{Debug, Formatter};

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::infer_type;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infra::option::OptionAnyExt;
use crate::parser::r#type::r#type::OptType;
use crate::parser::r#type::r#type::Type;

pub type OptExpr = Option<Expr>;

pub type StructField = (String, OptType, Expr);

#[derive(Clone, PartialEq)]
pub enum Expr {
    Unit(OptType),
    // TODO: Handle int overflow
    Int(OptType, i64),
    EnvRef(OptType, String),
    Closure(OptType, Option<String>, OptType, Box<Expr>),
    Struct(OptType, Vec<StructField>),
    Discard(OptType),

    Cond(OptType, Box<Expr>, Box<Expr>, Box<Expr>),
    Match(OptType, Box<Expr>, Vec<(Expr, Expr)>),
    Apply(OptType, Box<Expr>, Box<Expr>),
    Let(OptType, String, OptType, Box<Expr>, Box<Expr>)
}

impl Expr {
    pub fn infer_type(
        &self,
        type_env: &TypeEnv,
        expr_env: &ExprEnv
    ) -> InferTypeRet {
        infer_type(type_env, expr_env, self)
    }

    pub fn with_fallback_type(&self, r#type: &Type) -> Expr {
        match &self {
            Expr::Unit(None) => Expr::Unit(r#type.clone().some()),
            Expr::Int(None, i) =>
                Expr::Int(r#type.clone().some(), i.clone()),
            Expr::EnvRef(None, r_n) =>
                Expr::EnvRef(r#type.clone().some(), r_n.to_string()),
            Expr::Apply(None, l_e, r_e) => Expr::Apply(
                r#type.clone().some(),
                l_e.clone(),
                r_e.clone()
            ),
            Expr::Cond(None, b_e, t_e, e_e) => Expr::Cond(
                r#type.clone().some(),
                b_e.clone(),
                t_e.clone(),
                e_e.clone()
            ),
            Expr::Closure(None, i_n, i_t, o_e) => Expr::Closure(
                r#type.clone().some(),
                i_n.clone(),
                i_t.clone(),
                o_e.clone()
            ),
            Expr::Struct(None, s_v) =>
                Expr::Struct(r#type.clone().some(), s_v.clone()),
            Expr::Discard(None) =>
                Expr::Discard(r#type.clone().some()),
            Expr::Match(None, t_e, c_v) => Expr::Match(
                r#type.clone().some(),
                t_e.clone(),
                c_v.clone()
            ),
            Expr::Let(None, a_n, a_t, a_e, s_e) => Expr::Let(
                r#type.clone().some(),
                a_n.to_string(),
                a_t.clone(),
                a_e.clone(),
                s_e.clone()
            ),
            _ => self.clone()
        }
    }

    pub fn with_opt_fallback_type(
        &self,
        r#type: &Option<Type>
    ) -> Expr {
        match r#type {
            Some(t) => self.with_fallback_type(t),
            None => self.clone()
        }
    }

    pub fn is_no_type_annot(&self) -> bool {
        match self {
            Expr::Unit(None) |
            Expr::Int(None, ..) |
            Expr::EnvRef(None, ..) |
            Expr::Apply(None, ..) |
            Expr::Cond(None, ..) |
            Expr::Closure(None, ..) |
            Expr::Struct(None, ..) |
            Expr::Discard(None) |
            Expr::Match(None, ..) |
            Expr::Let(None, ..) => true,
            _ => false
        }
    }

    pub fn is_fully_typed(&self) -> bool {
        match self {
            Expr::Unit(Some(_)) |
            Expr::Int(Some(_), ..) |
            Expr::EnvRef(Some(_), ..) |
            Expr::Discard(Some(_)) => true,

            Expr::Apply(Some(_), l_e, r_e) =>
                l_e.is_fully_typed() && r_e.is_fully_typed(),
            Expr::Cond(Some(_), b_e, t_e, e_e) =>
                b_e.is_fully_typed() &&
                    t_e.is_fully_typed() &&
                    e_e.is_fully_typed(),
            Expr::Closure(Some(_), _, Some(_), o_e) =>
                o_e.is_fully_typed(),
            Expr::Struct(Some(_), s_v) =>
                s_v.iter()
                    .all(|(_, sf_t, sf_e)| {
                        sf_t.is_some() && sf_e.is_fully_typed()
                    }),
            Expr::Match(Some(_), t_e, c_v) =>
                t_e.is_fully_typed() &&
                    c_v.iter().all(|(c_e, t_e)| {
                        c_e.is_fully_typed() && t_e.is_fully_typed()
                    }),
            Expr::Let(Some(_), _, Some(_), a_e, s_e) =>
                a_e.is_fully_typed() && s_e.is_fully_typed(),

            _ => false
        }
    }

    pub fn get_type_annot(&self) -> Option<&Type> {
        match self {
            Expr::Unit(Some(t)) => t,
            Expr::Int(Some(t), ..) => t,
            Expr::EnvRef(Some(t), ..) => t,
            Expr::Apply(Some(t), ..) => t,
            Expr::Cond(Some(t), ..) => t,
            Expr::Closure(Some(t), ..) => t,
            Expr::Struct(Some(t), ..) => t,
            Expr::Discard(Some(t)) => t,
            Expr::Match(Some(t), ..) => t,
            Expr::Let(Some(t), ..) => t,
            _ => return None
        }
        .some()
    }

    pub fn unwrap_type_annot(&self) -> &Type {
        self.get_type_annot().unwrap()
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn type_annot(t: &OptType) -> String {
            match t {
                Some(t) => format!(":{t:?}"),
                None => format!("")
            }
        }

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

            Expr::EnvRef(t, r_n) =>
                f.write_str(&*format!("{r_n}{}", type_annot(t))),

            Expr::Apply(t, l_e, r_e) => match t {
                Some(t) => f.write_str(&*format!(
                    "(({l_e:?} {r_e:?}):{t:?})"
                )),
                None => f.write_str(&*format!("({l_e:?} {r_e:?})"))
            },

            Expr::Cond(t, b, t_e, e_e) => f.write_str(&*format!(
                "(if {b:?} then {t_e:?} else {e_e:?}){}",
                type_annot(t)
            )),

            Expr::Closure(t, i_n, i_t, o_e) =>
                f.write_str(&*format!(
                    "({}{} -> {o_e:?}){}",
                    closure_input_name(i_n),
                    type_annot(i_t),
                    type_annot(t)
                )),

            Expr::Struct(t, s_v) => f.write_str(&*format!(
                "{{ {s_v:?}{} }}",
                type_annot(t)
            )),

            Expr::Discard(t) =>
                f.write_str(&*format!("_{}", type_annot(t))),

            Expr::Match(t, t_e, c_v) => f.write_str(&*format!(
                "(match {t_e:?} with {c_v:?}){}",
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
