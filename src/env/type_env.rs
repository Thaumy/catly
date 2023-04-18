use std::rc::Rc;

use crate::parser::r#type::r#type::Type;

// 顶层类型环境
#[derive(Debug)]
pub struct TypeEnv {
    env: Rc<Vec<(String, Type)>>
}

impl Clone for TypeEnv {
    fn clone(&self) -> Self {
        TypeEnv {
            env: self.env.clone()
        }
    }
}

impl TypeEnv {
    pub fn new(vec: Vec<(String, Type)>) -> TypeEnv {
        println!("{:8}{:>10} │ {vec:?}", "[env]", "TypeEnv");
        TypeEnv { env: Rc::new(vec) }
    }

    pub fn find_type(&self, ref_name: &str) -> Option<&Type> {
        self.env
            .iter()
            .find(|(n, _)| n == ref_name)
            .map(|(_, t)| t)
    }

    pub fn exist_ref(&self, ref_name: &str) -> bool {
        let is_exist = match ref_name {
            "Int" | "Unit" => true,
            _ => self
                .env
                .iter()
                .any(|(n, _)| n == ref_name)
        };
        if !is_exist {
            println!("NamelyType '{ref_name}' not exist in type env",);
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
