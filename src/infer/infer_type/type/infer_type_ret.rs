use std::ops::{ControlFlow, FromResidual, Try};

use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::require_constraint::{
    require_constraint,
    ReqConstraint
};
use crate::infer::infer_type::r#type::require_info::ReqInfo;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::option::OptionAnyExt;
use crate::infra::quad::{Quad, QuadAnyExt};
use crate::infra::triple::Triple;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::OptType;
use crate::parser::r#type::r#type::Type;

pub type InferTypeRet =
    Quad<(Type, Expr), ReqConstraint, ReqInfo, TypeMissMatch>;

impl InferTypeRet {
    pub fn try_get_type(&self) -> OptType {
        match self.clone() {
            Quad::L((t, _)) |
            Quad::ML(ReqConstraint { r#type: t, .. }) => t.some(),
            _ => return None
        }
    }

    pub fn unwrap_type_constraint_expr(
        self
    ) -> (Type, EnvRefConstraint, Expr) {
        match self {
            Quad::L((input_type, typed_expr)) =>
                (input_type, EnvRefConstraint::empty(), typed_expr),
            Quad::ML(rc) => (rc.r#type, rc.constraint, rc.typed_expr),
            _ => panic!("Impossible value: {self:?}")
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
            constraint.unwrap_or_else(|| EnvRefConstraint::empty());

        if from.is_partial() {
            return ReqInfo::of("(partial type)", constraint).into();
        };

        match to {
            Some(to) => match from.lift_to(type_env, to) {
                Some(t) => require_constraint(
                    t.clone(),
                    constraint,
                    typed_expr(t)
                ),
                None => TypeMissMatch::of_type(from, to).into()
            },
            None => require_constraint(
                from.clone(),
                constraint,
                typed_expr(from.clone())
            )
        }
    }

    pub fn has_type(r#type: Type, typed_expr: Expr) -> InferTypeRet {
        (r#type, typed_expr).quad_l()
    }
}

impl Triple<(Type, Expr), ReqConstraint, ReqInfo> {
    pub fn unwrap_type_constraint_expr(
        self
    ) -> (Type, EnvRefConstraint, Expr) {
        match self {
            Triple::L((input_type, typed_expr)) =>
                (input_type, EnvRefConstraint::empty(), typed_expr),
            Triple::M(rc) =>
                (rc.r#type, rc.constraint, rc.typed_expr),
            _ => panic!("Impossible value: {self:?}")
        }
    }

    pub fn with_constraint_acc(
        self,
        constraint_acc: EnvRefConstraint
    ) -> InferTypeRet {
        match self {
            Triple::L((t, typed_expr)) =>
                require_constraint(t, constraint_acc, typed_expr),
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
                r#type: rc.r#type,
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

impl From<Triple<(Type, Expr), ReqConstraint, ReqInfo>>
    for InferTypeRet
{
    fn from(
        value: Triple<(Type, Expr), ReqConstraint, ReqInfo>
    ) -> Self {
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
    type Output = Triple<(Type, Expr), ReqConstraint, ReqInfo>;
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
