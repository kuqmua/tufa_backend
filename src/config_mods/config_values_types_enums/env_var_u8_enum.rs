use std::collections::HashMap;

use procedural_macros_lib::EnumVariantCount;

use strum::IntoEnumIterator;

use strum_macros::EnumIter;

use dotenv::dotenv;

use crate::config_mods::config_error_mods::config_env_var_error_type_enum::ConfigEnvVarErrorType;
use crate::config_mods::config_error_mods::config_error::ConfigError;
use crate::config_mods::config_error_mods::config_error_inner_type_enum::ConfigErrorInnerType;
use crate::config_mods::config_error_mods::var_or_int_parse_error_enum::VarOrIntParseError;

use crate::constants::project_constants::ENV_FILE_NAME;

#[derive(
    EnumVariantCount,
    EnumIter,
    Clone,
    Debug,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    PartialEq,
    Eq,
    Hash,
    Copy,
)]
pub enum EnvU8Var {
    ErrorRed,
    ErrorGreen,
    ErrorBlue,
    WarningHighRed,
    WarningHighGreen,
    WarningHighBlue,
    WarningLowRed,
    WarningLowGreen,
    WarningLowBlue,
    SuccessRed,
    SuccessGreen,
    SuccessBlue,
    PartialSuccessRed,
    PartialSuccessGreen,
    PartialSuccessBlue,
    CleaningRed,
    CleaningGreen,
    CleaningBlue,
    TimeMeasurementRed,
    TimeMeasurementGreen,
    TimeMeasurementBlue,
    InfoRed,
    InfoGreen,
    InfoBlue,
}

impl EnvU8Var {
    pub fn get_length() -> usize {
        ENUM_LENGTH
    }
    pub fn into_string_name_and_kind_tuple_vec() -> Vec<(&'static str, EnvU8Var)> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvU8Var::get_length());
        for env_var_name_kind in EnvU8Var::iter() {
            env_var_name_kind_vec
                .push((EnvU8Var::get_env_name(env_var_name_kind), env_var_name_kind));
        }
        env_var_name_kind_vec
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn into_string_name_and_kind_hashmap() -> HashMap<&'static str, EnvU8Var> {
        let mut config_env_var_name_kind_string_to_enum_struct_hasmap: HashMap<
            &'static str,
            EnvU8Var,
        > = HashMap::with_capacity(EnvU8Var::get_length());
        for env_var_name_kind_kind in EnvU8Var::iter() {
            config_env_var_name_kind_string_to_enum_struct_hasmap.insert(
                EnvU8Var::get_env_name(env_var_name_kind_kind),
                env_var_name_kind_kind,
            );
        }
        config_env_var_name_kind_string_to_enum_struct_hasmap
    }
    pub fn get_string_from_env_var(
        env_var_name_kind: EnvU8Var,
        was_dotenv_enable: bool,
    ) -> Result<String, ConfigError<'static>> {
        let string_name = EnvU8Var::get_env_name(env_var_name_kind);
        match std::env::var(string_name) {
            Ok(handle) => Ok(handle),
            Err(e) => Err(ConfigError {
                env_var_name_kind: ConfigEnvVarErrorType::U8(env_var_name_kind),
                was_dotenv_enable,
                env_name: string_name,
                env_error: ConfigErrorInnerType::VarErrorHandle(e),
            }),
        }
    }
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
                            env_name: EnvU8Var::get_env_name(env_var_name_kind),
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
