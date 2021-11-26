use crate::config_mods::env_var_enum::EnvVar;

use crate::config_mods::config_values_types_enums::env_var_u8_enum::EnvU8Var;

impl EnvU8Var {
    pub fn get_env_name(env_var_name_kind: EnvU8Var) -> &'static str {
        match env_var_name_kind {
            EnvU8Var::ErrorRed => EnvVar::get_env_name(EnvVar::ErrorRed),
            EnvU8Var::ErrorGreen => EnvVar::get_env_name(EnvVar::ErrorGreen),
            EnvU8Var::ErrorBlue => EnvVar::get_env_name(EnvVar::ErrorBlue),
            EnvU8Var::WarningHighRed => EnvVar::get_env_name(EnvVar::WarningHighRed),
            EnvU8Var::WarningHighGreen => EnvVar::get_env_name(EnvVar::WarningHighGreen),
            EnvU8Var::WarningHighBlue => EnvVar::get_env_name(EnvVar::WarningHighBlue),
            EnvU8Var::WarningLowRed => EnvVar::get_env_name(EnvVar::WarningLowRed),
            EnvU8Var::WarningLowGreen => EnvVar::get_env_name(EnvVar::WarningLowGreen),
            EnvU8Var::WarningLowBlue => EnvVar::get_env_name(EnvVar::WarningLowBlue),
            EnvU8Var::SuccessRed => EnvVar::get_env_name(EnvVar::SuccessRed),
            EnvU8Var::SuccessGreen => EnvVar::get_env_name(EnvVar::SuccessGreen),
            EnvU8Var::SuccessBlue => EnvVar::get_env_name(EnvVar::SuccessBlue),
            EnvU8Var::PartialSuccessRed => EnvVar::get_env_name(EnvVar::PartialSuccessRed),
            EnvU8Var::PartialSuccessGreen => EnvVar::get_env_name(EnvVar::PartialSuccessGreen),
            EnvU8Var::PartialSuccessBlue => EnvVar::get_env_name(EnvVar::PartialSuccessBlue),
            EnvU8Var::CleaningRed => EnvVar::get_env_name(EnvVar::CleaningRed),
            EnvU8Var::CleaningGreen => EnvVar::get_env_name(EnvVar::CleaningGreen),
            EnvU8Var::CleaningBlue => EnvVar::get_env_name(EnvVar::CleaningBlue),
            EnvU8Var::TimeMeasurementRed => EnvVar::get_env_name(EnvVar::TimeMeasurementRed),
            EnvU8Var::TimeMeasurementGreen => EnvVar::get_env_name(EnvVar::TimeMeasurementGreen),
            EnvU8Var::TimeMeasurementBlue => EnvVar::get_env_name(EnvVar::TimeMeasurementBlue),
            EnvU8Var::InfoRed => EnvVar::get_env_name(EnvVar::InfoRed),
            EnvU8Var::InfoGreen => EnvVar::get_env_name(EnvVar::InfoGreen),
            EnvU8Var::InfoBlue => EnvVar::get_env_name(EnvVar::InfoBlue),
        }
    }
}
