use crate::infer::env::r#macro::int_type;
use crate::infer::env::r#macro::unit_type;
use crate::infra::option::WrapOption;
use crate::parser::r#type::r#type::Type;

pub type TypeEnvEntry = (String, Type);

// 编译时类型环境
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

        #[cfg(feature = "ct_env_log")]
        {
            let log = format!(
                "{:8}{:>10} │ {:?}",
                "[ct env]", "TypeEnv", type_env.env
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
                .wrap_some(),
            env: type_vec
        };

        #[cfg(feature = "ct_env_log")]
        {
            let log = format!(
                "{:8}{:>10} │ {:?}",
                "[ct env]", "TypeEnv", type_env.env
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
            (Some(entry), _) => entry.wrap_some(),
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
            "Int" => int_type!().wrap_some(),
            "Unit" => unit_type!().wrap_some(),
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
        if !is_exist && cfg!(feature = "ct_env_log") {
            let log = format!(
                "NamelyType '{type_name}' not exist in type env"
            );
            println!("{log}");
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
