use std::fmt::{Debug, Formatter};

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::infer_type;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infra::option::AnyExt;
use crate::parser::r#type::r#type::MaybeType;
use crate::parser::r#type::r#type::Type;

pub type MaybeExpr = Option<Expr>;

pub type StructField = (String, MaybeType, Expr);

#[derive(Clone, PartialEq)]
pub enum Expr {
    Unit(MaybeType),
    // TODO: Handle int overflow
    Int(MaybeType, i64),
    EnvRef(MaybeType, String),
    Apply(MaybeType, Box<Expr>, Box<Expr>),
    Cond(MaybeType, Box<Expr>, Box<Expr>, Box<Expr>),
    Closure(MaybeType, Option<String>, MaybeType, Box<Expr>),
    Struct(MaybeType, Vec<StructField>),
    Discard(MaybeType),
    Match(MaybeType, Box<Expr>, Vec<(Expr, Expr)>),
    Let(MaybeType, String, MaybeType, Box<Expr>, Box<Expr>)
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
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn type_annot(t: &MaybeType) -> String {
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
