use std::rc::Rc;

use crate::eval::r#type::r#type::Type;
use crate::infra::option::AnyExt;
use crate::infra::r#box::Ext;

// 运行时类型环境
#[derive(Clone, Debug)]
pub struct TypeEnv<'t> {
    prev_env: Option<&'t TypeEnv<'t>>,
    env: Vec<(String, Type)>
}

impl<'t> TypeEnv<'t> {
    pub fn new(type_vec: Vec<(String, Type)>) -> TypeEnv<'t> {
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

    pub fn extend_new(
        &self,
        type_vec: Vec<(String, Type)>
    ) -> TypeEnv {
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
    ) -> Option<&(String, Type)> {
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
}
