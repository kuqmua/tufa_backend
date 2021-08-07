use crate::get_project_information::get_config::structures_definitions::env_def::Env;

impl From<&str> for Env {
    fn from(env: &str) -> Self {
        match env {
            "Development" => Env::Development,
            "Production" => Env::Production,
            "Testing" => Env::Testing,
            _ => Env::Development,
        }
    }
}
