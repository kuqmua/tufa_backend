use std::collections::HashMap;

use strum::IntoEnumIterator;

use dotenv::dotenv;

use crate::config_mods::config_error_mods::config_env_var_error_type_enum::ConfigEnvVarErrorType;
use crate::config_mods::config_error_mods::config_error::ConfigError;
use crate::config_mods::config_error_mods::config_error_inner_type_enum::ConfigErrorInnerType;
use crate::config_mods::config_error_mods::var_or_bool_parse_error_enum::VarOrBoolParseError;
use crate::config_mods::config_values_types_enums::env_var_bool_enum::EnvBoolVar;

use crate::constants::project_constants::ENV_FILE_NAME;

use crate::traits::get_env_name_trait::GetEnvName;

impl EnvBoolVar {
    pub fn get_env_values_hashmap() -> Result<HashMap<EnvBoolVar, bool>, ConfigError<'static>> {
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
        let mut hmap: HashMap<EnvBoolVar, bool> = HashMap::new();
        let mut error_option: Option<ConfigError> = None;
        for env_var_name_kind in EnvBoolVar::iter() {
            match EnvBoolVar::get_string_from_env_var(env_var_name_kind, was_dotenv_enable) {
                Ok(env_var_string) => match env_var_string.parse::<bool>() {
                    Ok(handle) => {
                        hmap.insert(env_var_name_kind, handle);
                    }
                    Err(e) => {
                        error_option = Some(ConfigError {
                            env_var_name_kind: ConfigEnvVarErrorType::Bool(env_var_name_kind),
                            was_dotenv_enable,
                            env_name: env_var_name_kind.get_env_name(),
                            env_error: ConfigErrorInnerType::VarOrBoolParseErrorHandle(
                                VarOrBoolParseError::Bool(e),
                            ),
                        });
                        break;
                    }
                },
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
