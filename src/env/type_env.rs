use crate::parser::r#type::Type;

// 顶层类型环境
pub struct TypeEnv {
    env: Vec<(String, Type)>
}

impl TypeEnv {
    pub fn new(vec: Vec<(String, Type)>) -> TypeEnv {
        TypeEnv { env: vec }
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
            println!(
                "TypeEnvRef {:?} not exist in type env",
                ref_name
            );
        }
        is_exist
    }
}
