use std::collections::BTreeSet;

use crate::eval::r#type::r#type::Type;
use crate::infra::option::OptionAnyExt;

pub type TypeEnvEntry = (String, Type);

// 运行时类型环境
#[derive(Clone, Debug)]
pub struct TypeEnv<'t> {
    prev_env: Option<&'t TypeEnv<'t>>,
    env: Vec<TypeEnvEntry>
}

impl<'t> TypeEnv<'t> {
    pub fn new(type_vec: Vec<TypeEnvEntry>) -> TypeEnv<'t> {
        let type_env = TypeEnv {
            prev_env: None,
            env: type_vec
        };

        if cfg!(feature = "rt_env_log") {
            let log = format!(
                "{:8}{:>10} │ {:?}",
                "[rt env]", "TypeEnv", type_env.env
            );
            println!("{log}");
        }

        type_env
    }

    fn latest_none_empty_type_env(&self) -> &TypeEnv {
        match (self.env.is_empty(), &self.prev_env) {
            (true, Some(prev_env)) =>
                prev_env.latest_none_empty_type_env(),
            _ => self
        }
    }

    pub fn extend_new(&self, type_vec: Vec<TypeEnvEntry>) -> TypeEnv {
        let type_env = TypeEnv {
            prev_env: self
                .latest_none_empty_type_env()
                .some(),
            env: type_vec
        };

        if cfg!(feature = "rt_env_log") {
            let log = format!(
                "{:8}{:>10} │ {:?}",
                "[rt env]", "TypeEnv", type_env.env
            );
            println!("{log}");
        }

        type_env
    }

    fn find_entry<'s>(
        &self,
        type_name: impl Into<&'s str>
    ) -> Option<&TypeEnvEntry> {
        let type_name = type_name.into();
        let entry = self
            .env
            .iter()
            .rev()
            .find(|(n, ..)| n == type_name);

        match (entry, &self.prev_env) {
            (Some(entry), _) => entry.some(),
            (None, Some(prev_env)) => prev_env.find_entry(type_name),
            _ => None
        }
    }

    pub fn find_type<'s>(
        &self,
        type_name: impl Into<&'s str>
    ) -> Option<Type> {
        let type_name = type_name.into();
        match type_name {
            "Int" => Type::NamelyType("Int".to_string()).some(),
            "Unit" => Type::NamelyType("Unit".to_string()).some(),
            _ => self
                .find_entry(type_name)
                .map(|(_, t)| t.clone())
        }
    }

    // 寻踪某个类型到和类型, 并返回这个和类型的构成
    // 目前仅由 can_lift_to 使用, 为后续扩展性设计而保留
    #[inline]
    fn source_sum_type(
        &self,
        r#type: &Type
    ) -> Option<BTreeSet<Type>> {
        match r#type {
            // 和编译期的类型提升规则一样, 不允许跨层寻踪
            Type::NamelyType(n) => match self.find_type(n.as_str()) {
                Some(Type::SumType(s)) => s.some(),
                _ => None
            },
            Type::SumType(s) => s.clone().some(),
            _ => None
        }
    }

    // 仅允许将类型提升到以它为基础的和类型, 这被用作 match 表达式的类型匹配
    pub fn can_lift_to(&self, from: &Type, to: &Type) -> bool {
        if from == to {
            return true;
        } else {
            match self.source_sum_type(to) {
                None => return false,
                Some(s) => s.iter().any(|t| t == from)
            }
        }
    }
}
