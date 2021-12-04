use crate::config_mods::config_error_mods::config_env_var_error_type_enum::ConfigEnvVarErrorType;
use crate::config_mods::config_error_mods::config_error::ConfigError;
use crate::config_mods::config_error_mods::config_error_inner_type_enum::ConfigErrorInnerType;

use crate::config_mods::config_values_types_enums::env_var_u8_enum::EnvU8Var;

use crate::traits::env_var_trait::EnvVarTrait;
use crate::traits::env_var_typed_trait::EnvVarTypedTrait;

impl EnvVarTypedTrait for EnvU8Var {
    fn get_string_from_env_var(
        &self,
        was_dotenv_enable: bool,
    ) -> Result<String, ConfigError<'static>> {
        let string_name = self.get_env_name();
        match std::env::var(string_name) {
            Ok(handle) => Ok(handle),
            Err(e) => Err(ConfigError {
                env_var_name_kind: ConfigEnvVarErrorType::U8(*self),
                was_dotenv_enable,
                env_name: string_name,
                env_error: ConfigErrorInnerType::VarErrorHandle(e),
            }),
        }
    }
}
