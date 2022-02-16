use crate::prints::print_type_enum::PrintType;
use crate::prints::print_wrapper::print_wrapper;

use crate::traits::print_type_trait::PrintTypeTrait;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[allow(clippy::too_many_arguments)]
pub fn handle_provider_prints(
    is_prints_enabled_provider: bool,
    is_error_prints_enabled_provider: bool,
    is_warning_high_prints_enabled_provider: bool,
    is_warning_low_prints_enabled_provider: bool,
    is_success_prints_enabled_provider: bool,
    is_partial_success_prints_enabled_provider: bool,
    enable_provider_time_measurement: bool,
    is_cleaning_warning_logs_directory_enabled_provider: bool,
    is_info_prints_enabled_provider: bool,
    pt: PrintType,
    sources_track: String,
    github_sources_track: String,
    message: String,
) {
    if is_prints_enabled_provider {
        match pt {
            PrintType::Error => {
                if pt.is_prints_enabled() && is_error_prints_enabled_provider {
                    print_wrapper(pt.get_color(), sources_track, github_sources_track, message);
                }
            }
            PrintType::WarningHigh => {
                if pt.is_prints_enabled() && is_warning_high_prints_enabled_provider
                {
                    print_wrapper(pt.get_color(), sources_track, github_sources_track, message);
                }
            }
            PrintType::WarningLow => {
                if pt.is_prints_enabled() && is_warning_low_prints_enabled_provider {
                    print_wrapper(pt.get_color(), sources_track, github_sources_track, message);
                }
            }
            PrintType::Success => {
                if pt.is_prints_enabled() && is_success_prints_enabled_provider {
                    print_wrapper(pt.get_color(), sources_track, github_sources_track, message);
                }
            }
            PrintType::PartialSuccess => {
                if pt.is_prints_enabled()
                    && is_partial_success_prints_enabled_provider
                {
                    print_wrapper(pt.get_color(), sources_track, github_sources_track, message);
                }
            }
            PrintType::TimeMeasurement => {
                if pt.is_prints_enabled() && enable_provider_time_measurement {
                    print_wrapper(pt.get_color(), sources_track, github_sources_track, message);
                }
            }
            PrintType::Info => {
                if pt.is_prints_enabled() && is_info_prints_enabled_provider {
                    print_wrapper(pt.get_color(), sources_track, github_sources_track, message);
                }
            }
        }
    }
}

