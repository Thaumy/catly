use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use crate::infra::option::AnyExt;
use crate::infra::r#fn::id;
use crate::parser::r#type::r#type::Type;

#[derive(PartialEq, Clone)]
pub struct EnvRefConstraint {
    constraint: HashMap<String, Type>
}

impl Debug for EnvRefConstraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("ERC::{:?}", self.constraint))
    }
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
                    // 允许精确类型替代不完整类型
                    Some(old_t) => t.eq_or_more_specific_than(&old_t)
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
                    // 允许精确类型替代不完整类型
                    Some(old_t) => t.eq_or_more_specific_than(&old_t)
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
        ref_name: impl Into<String>,
        r#type: Type
    ) -> EnvRefConstraint {
        EnvRefConstraint {
            constraint: HashMap::from([(ref_name.into(), r#type)])
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

    pub fn exclude_new<'s>(
        &self,
        ref_name: impl Into<&'s str>
    ) -> EnvRefConstraint {
        let ref_name = ref_name.into();
        self.filter_new(move |(n, _)| n.as_str() != ref_name)
    }

    pub fn is_empty(&self) -> bool { self.constraint.is_empty() }

    pub fn contains<'s>(&self, ref_name: impl Into<&'s str>) -> bool {
        self.constraint
            .contains_key(ref_name.into())
    }

    pub fn find<'s>(
        &self,
        ref_name: impl Into<&'s str>
    ) -> Option<&Type> {
        self.constraint
            .get(ref_name.into())
    }

    pub fn iter(&self) -> Iter<'_, String, Type> {
        self.constraint.iter()
    }
}
