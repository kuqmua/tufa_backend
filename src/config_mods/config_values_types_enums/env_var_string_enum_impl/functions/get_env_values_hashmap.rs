use std::collections::HashMap;

use strum::IntoEnumIterator;

use dotenv::dotenv;

use crate::config_mods::config_error_mods::config_error::ConfigError;

use crate::constants::project_constants::ENV_FILE_NAME;

use crate::config_mods::config_values_types_enums::env_var_string_enum::EnvStringVar;
use crate::traits::env_var_typed_trait::EnvVarTypedTrait;

impl EnvStringVar {
    pub fn get_env_values_hashmap() -> Result<HashMap<EnvStringVar, String>, ConfigError<'static>> {
        let was_dotenv_enable: bool;
        match dotenv() {
            Ok(_) => {
                was_dotenv_enable = true;
            }
            Err(e) => {
                was_dotenv_enable = false;
                println!(
                    "dotenv() failed, trying without {} error: {:?}",
                    ENV_FILE_NAME, e
                );
            }
        }
        let mut hmap: HashMap<EnvStringVar, String> = HashMap::new();
        let mut error_option: Option<ConfigError> = None;
        for env_var_name_kind in EnvStringVar::iter() {
            match env_var_name_kind.get_string_from_env_var(was_dotenv_enable) {
                Ok(env_var_string) => {
                    hmap.insert(env_var_name_kind, env_var_string);
                }
                Err(e) => {
                    error_option = Some(e);
                    break;
                }
            }
        }
        if let Some(error) = error_option {
            return Err(error);
        }
        Ok(hmap)
    }
}
