use crate::config_mods::env_var_enum::EnvVar;

use crate::config_mods::config_values_types_enums::env_var_u8_enum::EnvU8Var;

use crate::traits::get_env_name_trait::GetEnvName;

impl GetEnvName for EnvU8Var {
    fn get_env_name(self: &EnvU8Var) -> &'static str {
        match self {
            EnvU8Var::ErrorRed => EnvVar::ErrorRed.get_env_name(),
            EnvU8Var::ErrorGreen => EnvVar::ErrorGreen.get_env_name(),
            EnvU8Var::ErrorBlue => EnvVar::ErrorBlue.get_env_name(),
            EnvU8Var::WarningHighRed => EnvVar::WarningHighRed.get_env_name(),
            EnvU8Var::WarningHighGreen => EnvVar::WarningHighGreen.get_env_name(),
            EnvU8Var::WarningHighBlue => EnvVar::WarningHighBlue.get_env_name(),
            EnvU8Var::WarningLowRed => EnvVar::WarningLowRed.get_env_name(),
            EnvU8Var::WarningLowGreen => EnvVar::WarningLowGreen.get_env_name(),
            EnvU8Var::WarningLowBlue => EnvVar::WarningLowBlue.get_env_name(),
            EnvU8Var::SuccessRed => EnvVar::SuccessRed.get_env_name(),
            EnvU8Var::SuccessGreen => EnvVar::SuccessGreen.get_env_name(),
            EnvU8Var::SuccessBlue => EnvVar::SuccessBlue.get_env_name(),
            EnvU8Var::PartialSuccessRed => EnvVar::PartialSuccessRed.get_env_name(),
            EnvU8Var::PartialSuccessGreen => EnvVar::PartialSuccessGreen.get_env_name(),
            EnvU8Var::PartialSuccessBlue => EnvVar::PartialSuccessBlue.get_env_name(),
            EnvU8Var::CleaningRed => EnvVar::CleaningRed.get_env_name(),
            EnvU8Var::CleaningGreen => EnvVar::CleaningGreen.get_env_name(),
            EnvU8Var::CleaningBlue => EnvVar::CleaningBlue.get_env_name(),
            EnvU8Var::TimeMeasurementRed => EnvVar::TimeMeasurementRed.get_env_name(),
            EnvU8Var::TimeMeasurementGreen => EnvVar::TimeMeasurementGreen.get_env_name(),
            EnvU8Var::TimeMeasurementBlue => EnvVar::TimeMeasurementBlue.get_env_name(),
            EnvU8Var::InfoRed => EnvVar::InfoRed.get_env_name(),
            EnvU8Var::InfoGreen => EnvVar::InfoGreen.get_env_name(),
            EnvU8Var::InfoBlue => EnvVar::InfoBlue.get_env_name(),
        }
    }
    fn into_array() -> &'static [Self] {
        EnvU8Var::all_variants()
    }
}
