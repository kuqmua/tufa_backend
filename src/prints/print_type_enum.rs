use crate::traits::print_type_trait::PrintTypeTrait;

use crate::config_mods::lazy_static_config::CONFIG;

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

impl PrintTypeTrait for PrintType {
    fn is_prints_enabled(&self) -> bool {
        match *self {
            PrintType::Error => CONFIG.is_error_prints_enabled,
            PrintType::WarningHigh => CONFIG.is_warning_high_prints_enabled,
            PrintType::WarningLow => CONFIG.is_warning_low_prints_enabled,
            PrintType::Success => CONFIG.is_success_prints_enabled,
            PrintType::PartialSuccess => CONFIG.is_partial_success_prints_enabled,
            PrintType::TimeMeasurement => CONFIG.is_time_measurement_prints_enabled,
            PrintType::CleaningWarningLogsDirectory => CONFIG.is_cleaning_warning_logs_directory_enabled,
            PrintType::Info => CONFIG.is_info_prints_enabled,
        }
    }
}


