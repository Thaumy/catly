use std::ops::{ControlFlow, FromResidual, Try};

use crate::infer::env::TypeEnv;
use crate::infer::infer_type::EnvRefConstraint;
use crate::infer::infer_type::ReqInfo;
use crate::infer::infer_type::TypeMissMatch;
use crate::infer::infer_type::{require_constraint, ReqConstraint};
use crate::infra::Triple;
use crate::infra::{Quad, WrapQuad};
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::OptType;
use crate::parser::r#type::Type;

pub type InferTypeRet =
    Quad<Expr, ReqConstraint, ReqInfo, TypeMissMatch>;

impl InferTypeRet {
    pub fn unwrap_expr_constraint(self) -> (Expr, EnvRefConstraint) {
        match self {
            Quad::L(typed_expr) =>
                (typed_expr, EnvRefConstraint::empty()),
            Quad::ML(rc) => (rc.typed_expr, rc.constraint),
            _ => unreachable!()
        }
    }

    pub fn from_auto_lift<F>(
        type_env: &TypeEnv,
        from: &Type,
        to: &OptType,
        constraint: Option<EnvRefConstraint>,
        typed_expr: F
    ) -> InferTypeRet
    where
        F: Fn(Type) -> Expr
    {
        let constraint =
            constraint.unwrap_or_else(EnvRefConstraint::empty);

        if from.is_partial() {
            return ReqInfo::of("(partial type)", constraint).into();
        };

        match to {
            Some(to) => match from.lift_to(type_env, to) {
                Some(t) =>
                    require_constraint(typed_expr(t), constraint),
                None => TypeMissMatch::of_type(from, to).into()
            },
            None => require_constraint(
                typed_expr(from.clone()),
                constraint
            )
        }
    }

    pub fn has_type(typed_expr: Expr) -> InferTypeRet {
        typed_expr.wrap_quad_l()
    }
}

impl Triple<Expr, ReqConstraint, ReqInfo> {
    pub fn unwrap_expr_constraint(self) -> (Expr, EnvRefConstraint) {
        match self {
            Triple::L(typed_expr) =>
                (typed_expr, EnvRefConstraint::empty()),
            Triple::M(rc) => (rc.typed_expr, rc.constraint),
            _ => unreachable!()
        }
    }

    pub fn with_constraint_acc(
        self,
        constraint_acc: EnvRefConstraint
    ) -> InferTypeRet {
        match self {
            Triple::L(typed_expr) =>
                require_constraint(typed_expr, constraint_acc),
            Triple::M(rc) => rc.with_constraint_acc(constraint_acc),
            Triple::R(ri) => ri.with_constraint_acc(constraint_acc)
        }
    }

    pub fn exclude_constraint<'s>(
        self,
        ref_name: impl Into<&'s str>
    ) -> InferTypeRet {
        match self {
            Triple::M(rc) => ReqConstraint {
                constraint: rc
                    .constraint
                    .exclude_new(ref_name),
                typed_expr: rc.typed_expr
            }
            .into(),
            Triple::R(ri) => ReqInfo {
                ref_name: ri.ref_name,
                constraint: ri
                    .constraint
                    .exclude_new(ref_name)
            }
            .into(),
            other => other.into()
        }
    }

    pub fn intercept_req_info_name<'s>(
        self,
        new_ref_name: impl Into<&'s str>
    ) -> InferTypeRet {
        match self {
            Triple::R(ri) => ri
                .new_ref_name(new_ref_name.into())
                .into(),
            other => other.into()
        }
    }
}

impl From<Triple<Expr, ReqConstraint, ReqInfo>> for InferTypeRet {
    fn from(value: Triple<Expr, ReqConstraint, ReqInfo>) -> Self {
        match value {
            Triple::L(v) => Self::L(v),
            Triple::M(v) => Self::ML(v),
            Triple::R(v) => Self::MR(v)
        }
    }
}

impl FromResidual for InferTypeRet {
    #[inline]
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        residual
    }
}

impl Try for InferTypeRet {
    type Output = Triple<Expr, ReqConstraint, ReqInfo>;
    type Residual = InferTypeRet;

    #[inline]
    fn from_output(output: Self::Output) -> Self { output.into() }

    #[inline]
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::L(v) => ControlFlow::Continue(Triple::L(v)),
            Self::ML(v) => ControlFlow::Continue(Triple::M(v)),
            Self::MR(v) => ControlFlow::Continue(Triple::R(v)),
            e @ Self::R(_) => ControlFlow::Break(e)
        }
    }
}
