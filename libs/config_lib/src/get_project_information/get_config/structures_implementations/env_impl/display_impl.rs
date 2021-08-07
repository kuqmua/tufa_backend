use crate::get_project_information::get_config::structures_definitions::env_def::Env;
use std::fmt;

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Env::Development => write!(f, "Development"),
            Env::Production => write!(f, "Production"),
            Env::Testing => write!(f, "Testing"),
        }
    }
}
