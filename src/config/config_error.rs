use crate::config::config_error_inner_type_enum::ConfigErrorInnerType;
use crate::config::config_env_var_error_type_enum::ConfigEnvVarErrorType;

#[derive(Debug)]
pub struct ConfigError<'a> {
    pub env_var_name_kind: ConfigEnvVarErrorType,
    pub was_dotenv_enable: bool,
    pub env_name: &'a str,
    pub env_error: ConfigErrorInnerType,
}
