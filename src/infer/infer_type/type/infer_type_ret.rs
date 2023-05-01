use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::require_constraint::{
    require_constraint,
    RequireConstraint
};
use crate::infer::infer_type::r#type::require_info::RequireInfo;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
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
