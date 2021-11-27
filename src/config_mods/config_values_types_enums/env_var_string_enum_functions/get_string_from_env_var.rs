use crate::config_mods::config_error_mods::config_env_var_error_type_enum::ConfigEnvVarErrorType;
use crate::config_mods::config_error_mods::config_error::ConfigError;
use crate::config_mods::config_error_mods::config_error_inner_type_enum::ConfigErrorInnerType;

use crate::config_mods::config_values_types_enums::env_var_string_enum::EnvStringVar;

impl EnvStringVar {
    pub fn get_string_from_env_var(
        env_var_name_kind: EnvStringVar,
        was_dotenv_enable: bool,
    ) -> Result<String, ConfigError<'static>> {
        let string_name = EnvStringVar::get_env_name(env_var_name_kind);
        match std::env::var(string_name) {
            Ok(handle) => Ok(handle),
            Err(e) => Err(ConfigError {
                env_var_name_kind: ConfigEnvVarErrorType::String(env_var_name_kind),
                was_dotenv_enable,
                env_name: string_name,
                env_error: ConfigErrorInnerType::VarErrorHandle(e),
            }),
        }
    }
}
