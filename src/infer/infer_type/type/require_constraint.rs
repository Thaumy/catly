use std::fmt::{Debug, Formatter};

use crate::infer::infer_type::EnvRefConstraint;
use crate::infer::infer_type::InferTypeRet;
use crate::infer::infer_type::TypeMissMatch;
use crate::infra::quad::QuadAnyExt;
use crate::parser::expr::r#type::Expr;

// 经由约束才能使用的类型
#[derive(PartialEq, Clone)]
pub struct ReqConstraint {
    pub typed_expr: Expr,
    pub constraint: EnvRefConstraint
}

impl ReqConstraint {
    pub fn with_constraint_acc(
        self,
        acc: EnvRefConstraint
    ) -> InferTypeRet {
        // 理论上约束顺序不会对环境造成影响, 因为在某一层环境产生的约束作用于其上层环境中最近注册的引用源
        // 而将这种约束注入最接近的上层环境不会改变这一就近原则
        // 实际上各项测试的通过也能说明目前由哈希表实现的约束结构是可行的
        require_extended_constraint(
            self.typed_expr,
            acc,
            self.constraint
        )
    }
}

pub fn require_constraint(
    typed_expr: Expr,
    constraint: EnvRefConstraint
) -> InferTypeRet {
    if constraint.is_empty() {
        InferTypeRet::has_type(typed_expr)
    } else {
        ReqConstraint {
            constraint,
            typed_expr
        }
        .into()
    }
}

pub fn require_extended_constraint(
    typed_expr: Expr,
    l: EnvRefConstraint,
    r: EnvRefConstraint
) -> InferTypeRet {
    match l.extend_new(r.clone()) {
        Some(c) => require_constraint(typed_expr, c),
        None => TypeMissMatch::of_constraint(&l, &r).into()
    }
}

impl From<ReqConstraint> for InferTypeRet {
    fn from(value: ReqConstraint) -> Self { value.quad_ml() }
}

impl Debug for ReqConstraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.constraint.is_empty() {
            f.write_str(&format!(
                "ReqC::{:?}",
                self.typed_expr
                    .get_type_annot()
            ))
        } else {
            f.write_str(&format!(
                "ReqC::{:?} & {:?}",
                self.typed_expr
                    .get_type_annot(),
                self.constraint
            ))
        }
    }
}
