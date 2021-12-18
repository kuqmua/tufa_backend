use std::collections::HashMap;

use crate::config_mods;
use crate::config_mods::config_error_mods::config_env_var_error_type_enum::ConfigEnvVarErrorType;
use crate::config_mods::config_error_mods::config_error::ConfigError;
// use crate::config_mods::config_error_mods::config_error_inner_type_enum::ConfigErrorInnerType;
// use crate::config_mods::config_error_mods::var_or_bool_parse_error_enum::VarOrBoolParseError;
use crate::config_mods::config_values_types_enums::env_var_bool_enum::EnvBoolVar;

use crate::traits::env_var_trait::EnvVarTrait;
use crate::traits::env_var_typed_trait::EnvVarTypedTrait;

use strum::IntoEnumIterator;

use dotenv::dotenv;

use crate::constants::project_constants::ENV_FILE_NAME;

impl EnvVarTypedTrait for EnvBoolVar {
    fn get_string_from_env_var(&self, was_dotenv_enable: bool) -> Result<String, ConfigError> {
        let string_name = self.get_env_name();
        match std::env::var(&string_name) {
            Ok(handle) => Ok(handle),
            Err(e) => Err(ConfigError {
                env_var_name_kind: ConfigEnvVarErrorType::Bool(*self),
                was_dotenv_enable,
                env_name: string_name,
                // env_error: ConfigErrorInnerType::VarErrorHandle(e),
            }),
        }
    }

    fn parse_string<T: std::str::FromStr>(value: String) -> Result<T, T::Err> {
        match value.parse::<T>() {
            Ok(handle) => Ok(handle),
            Err(e) => Err(e),
        }
    }

    fn get_env_values_hashmap<T: std::str::FromStr>() -> Result<HashMap<Self, T>, ConfigError> {
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
        let mut hmap: HashMap<Self, T> = HashMap::new();
        let mut error_option: Option<ConfigError> = None;
        for env_var_name_kind in Self::iter() {
            match env_var_name_kind.get_string_from_env_var(was_dotenv_enable) {
                Ok(env_var_string) => match config_mods::config_values_types_enums::env_var_bool_enum::EnvBoolVar::parse_string::<T>(env_var_string) {
                    Ok(handle) => {
                        hmap.insert(env_var_name_kind, handle);
                    },
                    Err(e) => {
                                error_option = Some(ConfigError {
                            env_var_name_kind: ConfigEnvVarErrorType::Bool(env_var_name_kind),
                            was_dotenv_enable,
                            env_name: env_var_name_kind.get_env_name(),
                            // env_error: ConfigErrorInnerType::VarOrIntParseErrorErrorHandle(
                            //     VarOrIntParseError::Int(e),
                            // ),
                        });
                        break;
                    },
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
