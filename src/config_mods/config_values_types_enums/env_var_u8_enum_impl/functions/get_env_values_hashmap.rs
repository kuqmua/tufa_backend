use std::collections::HashMap;

use strum::IntoEnumIterator;

use dotenv::dotenv;

use crate::config_mods::config_error_mods::config_env_var_error_type_enum::ConfigEnvVarErrorType;
use crate::config_mods::config_error_mods::config_error::ConfigError;
use crate::config_mods::config_error_mods::config_error_inner_type_enum::ConfigErrorInnerType;
use crate::config_mods::config_error_mods::var_or_int_parse_error_enum::VarOrIntParseError;

use crate::constants::project_constants::ENV_FILE_NAME;

use crate::config_mods::config_values_types_enums::env_var_u8_enum::EnvU8Var;

use crate::traits::env_var_trait::EnvVarTrait;

impl EnvU8Var {
    pub fn get_env_values_hashmap() -> Result<HashMap<EnvU8Var, u8>, ConfigError<'static>> {
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
        let mut hmap: HashMap<EnvU8Var, u8> = HashMap::new();
        let mut error_option: Option<ConfigError> = None;
        for env_var_name_kind in EnvU8Var::iter() {
            match EnvU8Var::get_string_from_env_var(env_var_name_kind, was_dotenv_enable) {
                Ok(env_var_string) => match env_var_string.parse::<u8>() {
                    Ok(handle) => {
                        hmap.insert(env_var_name_kind, handle);
                    }
                    Err(e) => {
                        error_option = Some(ConfigError {
                            env_var_name_kind: ConfigEnvVarErrorType::U8(env_var_name_kind),
                            was_dotenv_enable,
                            env_name: env_var_name_kind.get_env_name(),
                            env_error: ConfigErrorInnerType::VarOrIntParseErrorErrorHandle(
                                VarOrIntParseError::Int(e),
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
