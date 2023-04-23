use std::rc::Rc;

use crate::infra::option::AnyExt;
use crate::infra::r#box::Ext;
use crate::parser::r#type::r#type::Type;
use crate::{int_type, unit_type};

// 顶层类型环境
#[derive(Debug)]
pub struct TypeEnv {
    prev_env: Option<Box<TypeEnv>>,
    env: Rc<Vec<(String, Type)>>
}

impl Clone for TypeEnv {
    fn clone(&self) -> Self {
        TypeEnv {
            prev_env: self.prev_env.clone(),
            env: self.env.clone()
        }
    }
}

impl TypeEnv {
    pub fn new(type_vec: Vec<(String, Type)>) -> TypeEnv {
        let type_env = TypeEnv {
            prev_env: None,
            env: Rc::new(type_vec)
        };
        println!(
            "{:8}{:>10} │ {:?}",
            "[env]", "TypeEnv", type_env.env
        );
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
                .clone()
                .boxed()
                .some(),
            env: Rc::new(type_vec)
        };
        println!(
            "{:8}{:>10} │ {:?}",
            "[env]", "TypeEnv", type_env.env
        );
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
            "Int" => int_type!().some(),
            "Unit" => unit_type!().some(),
            _ => self
                .find_entry(type_name)
                .map(|(_, t)| t.clone())
        }
    }

    pub fn exist_ref<'s>(
        &self,
        type_name: impl Into<&'s str>
    ) -> bool {
        let type_name = type_name.into();
        let is_exist = match type_name {
            "Int" | "Unit" => true,
            _ => self
                .find_entry(type_name)
                .is_some()
        };
        if !is_exist {
            println!(
                "NamelyType '{type_name}' not exist in type env"
            );
        }
        is_exist
    }

    pub fn is_type_valid(&self, r#type: &Type) -> bool {
        match r#type {
            Type::NamelyType(type_name) =>
                self.exist_ref(type_name.as_str()),
            Type::ClosureType(input_type, output_type) =>
                self.is_type_valid(input_type) &&
                    self.is_type_valid(output_type),
            Type::ProdType(vec) => vec
                .iter()
                .all(|(_, t)| self.is_type_valid(t)),
            Type::SumType(set) => set
                .iter()
                .all(|t| self.is_type_valid(t)),
            // Partial types
            Type::PartialClosureType(input_type) =>
                self.is_type_valid(input_type),
        }
    }
}
