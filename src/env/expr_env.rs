use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::type_checker::r#type::TypeConstraint;

// 表达式环境
#[derive(Clone, Debug)]
pub struct ExprEnv<'t> {
    prev_env: Option<&'t ExprEnv<'t>>,
    env: Vec<(String, TypeConstraint)>
}

impl<'t> ExprEnv<'t> {
    pub fn new(vec: Vec<(String, TypeConstraint)>) -> ExprEnv<'t> {
        ExprEnv {
            prev_env: None,
            env: vec
        }
    }

    pub fn extend_vec_new(
        &self,
        vec: Vec<(String, TypeConstraint)>
    ) -> ExprEnv {
        ExprEnv {
            prev_env: Some(self),
            env: vec
        }
    }

    pub fn extend_new(
        &self,
        ref_name: String,
        tc: MaybeType
    ) -> ExprEnv {
        let tc = tc
            .map(|t| TypeConstraint::Constraint(t))
            .unwrap_or(TypeConstraint::Free);

        self.extend_vec_new(vec![(ref_name, tc)])
    }

    pub fn find_type(
        &self,
        ref_name: &str
    ) -> Option<&TypeConstraint> {
        match self
            .env
            .iter()
            .rev()
            .find(|(n, _)| n == ref_name)
            .map(|(_, t)| t)
        {
            Some(t) => t.some(),
            None => match self.prev_env {
                Some(env) => env.find_type(ref_name),
                None => None
            }
        }
    }

    pub fn exist_ref(&self, ref_name: &str) -> bool {
        match self
            .env
            .iter()
            .rev()
            .any(|(n, _)| n == ref_name)
        {
            true => true,
            false => match self.prev_env {
                Some(env) => env.exist_ref(ref_name),
                None => false
            }
        }
    }
}
