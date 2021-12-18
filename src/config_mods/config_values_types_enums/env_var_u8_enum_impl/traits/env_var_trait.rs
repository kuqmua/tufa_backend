use crate::config_mods::env_var_enum::EnvVar;

use crate::config_mods::config_values_types_enums::env_var_u8_enum::EnvU8Var;

use crate::traits::env_var_trait::EnvVarTrait;

impl EnvVarTrait for EnvU8Var {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_env_name(&self) -> &'static str {
        match self {
            Self::ErrorRed => EnvVar::ErrorRed.get_env_name(),
            Self::ErrorGreen => EnvVar::ErrorGreen.get_env_name(),
            Self::ErrorBlue => EnvVar::ErrorBlue.get_env_name(),
            Self::WarningHighRed => EnvVar::WarningHighRed.get_env_name(),
            Self::WarningHighGreen => EnvVar::WarningHighGreen.get_env_name(),
            Self::WarningHighBlue => EnvVar::WarningHighBlue.get_env_name(),
            Self::WarningLowRed => EnvVar::WarningLowRed.get_env_name(),
            Self::WarningLowGreen => EnvVar::WarningLowGreen.get_env_name(),
            Self::WarningLowBlue => EnvVar::WarningLowBlue.get_env_name(),
            Self::SuccessRed => EnvVar::SuccessRed.get_env_name(),
            Self::SuccessGreen => EnvVar::SuccessGreen.get_env_name(),
            Self::SuccessBlue => EnvVar::SuccessBlue.get_env_name(),
            Self::PartialSuccessRed => EnvVar::PartialSuccessRed.get_env_name(),
            Self::PartialSuccessGreen => EnvVar::PartialSuccessGreen.get_env_name(),
            Self::PartialSuccessBlue => EnvVar::PartialSuccessBlue.get_env_name(),
            Self::CleaningRed => EnvVar::CleaningRed.get_env_name(),
            Self::CleaningGreen => EnvVar::CleaningGreen.get_env_name(),
            Self::CleaningBlue => EnvVar::CleaningBlue.get_env_name(),
            Self::TimeMeasurementRed => EnvVar::TimeMeasurementRed.get_env_name(),
            Self::TimeMeasurementGreen => EnvVar::TimeMeasurementGreen.get_env_name(),
            Self::TimeMeasurementBlue => EnvVar::TimeMeasurementBlue.get_env_name(),
            Self::InfoRed => EnvVar::InfoRed.get_env_name(),
            Self::InfoGreen => EnvVar::InfoGreen.get_env_name(),
            Self::InfoBlue => EnvVar::InfoBlue.get_env_name(),
        }
    }
}
