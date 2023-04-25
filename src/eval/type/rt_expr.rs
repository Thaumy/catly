use std::fmt::{Debug, Formatter};

use crate::eval::r#type::rt_type::RtType;

pub type RtStructField = (String, RtType, RtExpr);

#[derive(Clone)]
pub enum RtExpr {
    Unit(RtType),
    Int(RtType, i64),
    EnvRef(RtType, String),
    Apply(RtType, Box<RtExpr>, Box<RtExpr>),
    Cond(RtType, Box<RtExpr>, Box<RtExpr>, Box<RtExpr>),
    Closure(RtType, Option<String>, RtType, Box<RtExpr>),
    Struct(RtType, Vec<RtStructField>),
    Discard(RtType),
    Match(RtType, Box<RtExpr>, Vec<(RtExpr, RtExpr)>),
    Let(RtType, String, RtType, Box<RtExpr>, Box<RtExpr>)
}

impl Debug for RtExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn type_annot(t: &RtType) -> String { format!(":{t:?}") }
        fn closure_input_name(s: &Option<String>) -> String {
            match s {
                Some(s) => format!("{s}"),
                None => format!("_")
            }
        }
        match self {
            RtExpr::Unit(t) =>
                f.write_str(&*format!("(){}", type_annot(t))),
            RtExpr::Int(t, i) =>
                f.write_str(&*format!("{i}{}", type_annot(t))),
            RtExpr::EnvRef(t, n) =>
                f.write_str(&*format!("{n}{}", type_annot(t))),
            RtExpr::Apply(t, l, r) =>
                f.write_str(&*format!("(({l:?} {r:?}):{t:?})")),
            RtExpr::Cond(t, b, te, fe) => f.write_str(&*format!(
                "(if {b:?} then {te:?} else {fe:?}){}",
                type_annot(t)
            )),
            RtExpr::Closure(t, i_n, i_t, o_e) =>
                f.write_str(&*format!(
                    "({}{} -> {o_e:?}){}",
                    closure_input_name(i_n),
                    type_annot(i_t),
                    type_annot(t)
                )),
            RtExpr::Struct(t, vec) => f.write_str(&*format!(
                "{{ {vec:?}{} }}",
                type_annot(t)
            )),
            RtExpr::Discard(t) =>
                f.write_str(&*format!("_{}", type_annot(t))),
            RtExpr::Match(t, t_e, vec) => f.write_str(&*format!(
                "(match {t_e:?} with {vec:?}){}",
                type_annot(t)
            )),
            RtExpr::Let(t, a_n, a_t, a_e, s_e) =>
                f.write_str(&*format!(
                    "(let {a_n}{} = {a_e:?} in {s_e:?}){}",
                    type_annot(a_t),
                    type_annot(t)
                )),
        }
    }
}
