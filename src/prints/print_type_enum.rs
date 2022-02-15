use crate::traits::print_type_from_config_trait::PrintTypeFromConfigTrait;

use crate::config_mods::lazy_static_config::CONFIG;


#[derive(PrintTypeFromConfigTraitDerive)]
pub enum PrintType {
    Error,
    WarningHigh,
    WarningLow,
    Success,
    PartialSuccess,
    TimeMeasurement,
    CleaningWarningLogsDirectory,
    Info,
}
