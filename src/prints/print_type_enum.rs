use crate::global_variables::runtime::config::CONFIG;
use ansi_term::Colour;
use ansi_term::Colour::RGB;
use tufa_common::traits::get_color::ErrorColor;
use tufa_common::traits::get_color::InfoColor;
use tufa_common::traits::get_color::PartialSuccessColor;
use tufa_common::traits::get_color::SuccessColor;
use tufa_common::traits::get_color::TimeMeasurementColor;
use tufa_common::traits::get_color::WarningHighColor;
use tufa_common::traits::get_color::WarningLowColor;
use tufa_common::traits::print_type_trait::PrintTypeTrait;

pub enum PrintType {
    Error,
    WarningHigh,
    WarningLow,
    Success,
    PartialSuccess,
    TimeMeasurement,
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
            PrintType::Info => CONFIG.is_info_prints_enabled,
        }
    }
    fn get_color(&self) -> Colour {
        match *self {
            PrintType::Error => CONFIG.get_error_color(),
            PrintType::WarningHigh => CONFIG.get_warning_high_color(),
            PrintType::WarningLow => CONFIG.get_warning_low_color(),
            PrintType::Success => CONFIG.get_success_color(),
            PrintType::PartialSuccess => CONFIG.get_partial_success_color(),
            PrintType::TimeMeasurement => CONFIG.get_time_measurement_color(),
            PrintType::Info => CONFIG.get_info_color(),
        }
    }
}
