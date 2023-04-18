use std::rc::Rc;

use crate::infra::option::AnyExt;
use crate::infra::r#box::Ext;
use crate::parser::r#type::r#type::Type;

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
        println!("{:8}{:>10} │ {type_vec:?}", "[env]", "TypeEnv");
        TypeEnv {
            prev_env: None,
            env: Rc::new(type_vec)
        }
    }

    pub fn extend_new(
        &self,
        type_vec: Vec<(String, Type)>
    ) -> TypeEnv {
        println!("{:8}{:>10} │ {type_vec:?}", "[env]", "TypeEnv");
        TypeEnv {
            prev_env: self.clone().boxed().some(),
            env: Rc::new(type_vec)
        }
    }

    fn find_entry(&self, type_name: &str) -> Option<&(String, Type)> {
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

    pub fn find_type(&self, type_name: &str) -> Option<&Type> {
        self.find_entry(type_name)
            .map(|(_, t)| t)
    }

    pub fn exist_ref(&self, type_name: &str) -> bool {
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
            Type::NamelyType(type_name) => self.exist_ref(type_name),
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
