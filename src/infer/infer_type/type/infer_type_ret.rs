use std::ops::{ControlFlow, FromResidual, Try};

use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::require_constraint::{
    require_constraint,
    RequireConstraint
};
use crate::infer::infer_type::r#type::require_info::RequireInfo;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::option::OptionAnyExt;
use crate::infra::quad::Quad;
use crate::infra::triple::Triple;
use crate::parser::r#type::r#type::OptType;
use crate::parser::r#type::r#type::Type;

pub type InferTypeRet =
    Quad<Type, RequireConstraint, RequireInfo, TypeMissMatch>;

impl InferTypeRet {
    pub fn try_get_type(&self) -> OptType {
        match self.clone() {
            Quad::ML(RequireConstraint { r#type: t, .. }) |
            Quad::L(t) => t.some(),
            _ => return None
        }
    }

    pub fn unwrap_type_and_constraint(
        self
    ) -> (Type, EnvRefConstraint) {
        match self {
            Quad::L(input_type) =>
                (input_type, EnvRefConstraint::empty()),
            Quad::ML(rc) => (rc.r#type, rc.constraint),
            _ => panic!("Impossible value: {self:?}")
        }
    }

    pub fn from_auto_lift(
        type_env: &TypeEnv,
        from: &Type,
        to: &OptType,
        constraint: Option<EnvRefConstraint>
    ) -> InferTypeRet {
        let constraint =
            constraint.unwrap_or_else(|| EnvRefConstraint::empty());

        if from.is_partial() {
            return RequireInfo::of("(partial type)", constraint)
                .into();
        };

        match to {
            Some(to) => match from.lift_to(type_env, to) {
                Some(t) => require_constraint(t, constraint),
                None => TypeMissMatch::of_type(from, to).into()
            },
            None => require_constraint(from.clone(), constraint)
        }
    }

    pub fn from_auto_unify(
        type_env: &TypeEnv,
        l: &Type,
        r: &Type,
        constraint: Option<EnvRefConstraint>
    ) -> InferTypeRet {
        let constraint =
            constraint.unwrap_or_else(|| EnvRefConstraint::empty());

        match l.unify(type_env, r) {
            Some(t) => require_constraint(t, constraint),
            None => TypeMissMatch::of_type(l, r).into()
        }
    }
}

impl From<InferTypeRet> for OptType {
    fn from(value: InferTypeRet) -> Self {
        match value {
            Quad::L(t) => t.some(),
            _ => None
        }
    }
}

impl Triple<Type, RequireConstraint, RequireInfo> {
    pub fn unwrap_type_and_constraint(
        self
    ) -> (Type, EnvRefConstraint) {
        match self {
            Triple::L(input_type) =>
                (input_type, EnvRefConstraint::empty()),
            Triple::M(rc) => (rc.r#type, rc.constraint),
            _ => panic!("Impossible value: {self:?}")
        }
    }
}

impl From<Triple<Type, RequireConstraint, RequireInfo>>
    for InferTypeRet
{
    fn from(
        value: Triple<Type, RequireConstraint, RequireInfo>
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
    type Output = Triple<Type, RequireConstraint, RequireInfo>;
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
