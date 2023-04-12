use std::collections::hash_map::Iter;
use std::collections::HashMap;

use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::infra::r#fn::id;
use crate::parser::r#type::Type;

// 经由约束才能使用的类型
#[derive(PartialEq, Clone, Debug)]
pub struct RequireConstraint {
    pub r#type: Type,
    pub constraint: EnvRefConstraint
}

#[derive(PartialEq, Clone, Debug)]
pub struct EnvRefConstraint {
    constraint: HashMap<String, Type>
}

impl EnvRefConstraint {
    pub fn extend_new(
        &self,
        other: EnvRefConstraint
    ) -> Option<EnvRefConstraint> {
        let mut hash_map = self.constraint.clone();
        if other
            .constraint
            .iter()
            .map(|(n, t)| {
                match hash_map.insert(n.to_string(), t.clone()) {
                    None => true,
                    Some(old_t) => &old_t == t
                }
            })
            .all(id)
        {
            EnvRefConstraint {
                constraint: hash_map
            }
            .some()
        } else {
            None
        }
    }

    pub fn new<const N: usize>(
        constraint: [(String, Type); N]
    ) -> Option<EnvRefConstraint> {
        let mut hash_map = HashMap::new();
        if constraint
            .iter()
            .map(|(n, t)| {
                match hash_map.insert(n.to_string(), t.clone()) {
                    None => true,
                    Some(old_t) => &old_t == t
                }
            })
            .all(id)
        {
            EnvRefConstraint {
                constraint: hash_map
            }
            .some()
        } else {
            None
        }
    }

    pub fn single(
        ref_name: String,
        r#type: Type
    ) -> EnvRefConstraint {
        EnvRefConstraint {
            constraint: HashMap::from([(ref_name, r#type)])
        }
    }

    pub fn empty() -> EnvRefConstraint {
        EnvRefConstraint {
            constraint: HashMap::new()
        }
    }

    pub fn filter_new<P>(&self, p: P) -> EnvRefConstraint
    where
        P: Fn((&String, &Type)) -> bool
    {
        EnvRefConstraint {
            constraint: self
                .constraint
                .iter()
                .filter(|(n, t)| p((n, t)))
                .map(|(n, t)| (n.clone(), t.clone()))
                .collect()
        }
    }

    pub fn is_empty(&self) -> bool { self.constraint.is_empty() }

    pub fn contains(&self, ref_name: &str) -> bool {
        self.constraint
            .contains_key(ref_name)
    }

    pub fn find(&self, ref_name: &str) -> Option<&Type> {
        self.constraint.get(ref_name)
    }

    pub fn iter(&self) -> Iter<'_, String, Type> {
        self.constraint.iter()
    }
}

// 需要类型信息
// 此情况由 env_ref case 产生时表明缺乏 ref_name 的类型信息
// discard case 产生该情况则表明某个弃元值缺乏关键的类型信息
#[derive(PartialEq, Clone, Debug)]
pub struct RequireInfo {
    pub ref_name: String
}

#[derive(PartialEq, Clone, Debug)]
pub struct TypeMissMatch {}

pub type GetTypeReturn =
    Quad<Type, RequireConstraint, RequireInfo, TypeMissMatch>;

impl From<GetTypeReturn> for Option<Type> {
    fn from(value: GetTypeReturn) -> Self {
        match value {
            Quad::L(t) => t.some(),
            _ => None
        }
    }
}
