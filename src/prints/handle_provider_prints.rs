use ansi_term::Colour::RGB;

use crate::config_mods::lazy_static_config::CONFIG;

use crate::prints::print_type_enum::PrintType;
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
                    let rgb_color: ansi_term::Colour =
                        RGB(CONFIG.error_red, CONFIG.error_green, CONFIG.error_blue);
                    if CONFIG.is_show_source_place_enabled
                        && CONFIG.is_show_github_source_place_enabled
                    {
                        eprintln!(
                            "{}{}\n{}",
                            rgb_color.bold().paint(sources_track),
                            rgb_color.bold().paint(github_sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else if CONFIG.is_show_source_place_enabled {
                        eprintln!(
                            "{}\n{}",
                            rgb_color.bold().paint(sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else if CONFIG.is_show_github_source_place_enabled {
                        eprintln!(
                            "{}\n{}",
                            rgb_color.bold().paint(github_sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else {
                        eprintln!("{}", rgb_color.bold().paint(message));
                    }
                }
            }
            PrintType::WarningHigh => {
                if pt.is_prints_enabled() && is_warning_high_prints_enabled_provider
                {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.warning_high_red,
                        CONFIG.warning_high_green,
                        CONFIG.warning_high_blue,
                    );
                    if CONFIG.is_show_source_place_enabled
                        && CONFIG.is_show_github_source_place_enabled
                    {
                        eprintln!(
                            "{}{}\n{}",
                            rgb_color.bold().paint(sources_track),
                            rgb_color.bold().paint(github_sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else if CONFIG.is_show_source_place_enabled {
                        eprintln!(
                            "{}\n{}",
                            rgb_color.bold().paint(sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else if CONFIG.is_show_github_source_place_enabled {
                        eprintln!(
                            "{}\n{}",
                            rgb_color.bold().paint(github_sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else {
                        eprintln!("{}", rgb_color.bold().paint(message));
                    }
                }
            }
            PrintType::WarningLow => {
                if pt.is_prints_enabled() && is_warning_low_prints_enabled_provider {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.warning_low_red,
                        CONFIG.warning_low_green,
                        CONFIG.warning_low_blue,
                    );
                    if CONFIG.is_show_source_place_enabled
                        && CONFIG.is_show_github_source_place_enabled
                    {
                        eprintln!(
                            "{}{}\n{}",
                            rgb_color.bold().paint(sources_track),
                            rgb_color.bold().paint(github_sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else if CONFIG.is_show_source_place_enabled {
                        eprintln!(
                            "{}\n{}",
                            rgb_color.bold().paint(sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else if CONFIG.is_show_github_source_place_enabled {
                        eprintln!(
                            "{}\n{}",
                            rgb_color.bold().paint(github_sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else {
                        eprintln!("{}", rgb_color.bold().paint(message));
                    }
                }
            }
            PrintType::Success => {
                if pt.is_prints_enabled() && is_success_prints_enabled_provider {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.success_red,
                        CONFIG.success_green,
                        CONFIG.success_blue,
                    );
                    if CONFIG.is_show_source_place_enabled
                        && CONFIG.is_show_github_source_place_enabled
                    {
                        eprintln!(
                            "{}{}\n{}",
                            rgb_color.bold().paint(sources_track),
                            rgb_color.bold().paint(github_sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else if CONFIG.is_show_source_place_enabled {
                        eprintln!(
                            "{}\n{}",
                            rgb_color.bold().paint(sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else if CONFIG.is_show_github_source_place_enabled {
                        eprintln!(
                            "{}\n{}",
                            rgb_color.bold().paint(github_sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else {
                        eprintln!("{}", rgb_color.bold().paint(message));
                    }
                }
            }
            PrintType::PartialSuccess => {
                if pt.is_prints_enabled()
                    && is_partial_success_prints_enabled_provider
                {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.partial_success_red,
                        CONFIG.partial_success_green,
                        CONFIG.partial_success_blue,
                    );
                    if CONFIG.is_show_source_place_enabled
                        && CONFIG.is_show_github_source_place_enabled
                    {
                        eprintln!(
                            "{}{}\n{}",
                            rgb_color.bold().paint(sources_track),
                            rgb_color.bold().paint(github_sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else if CONFIG.is_show_source_place_enabled {
                        eprintln!(
                            "{}\n{}",
                            rgb_color.bold().paint(sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else if CONFIG.is_show_github_source_place_enabled {
                        eprintln!(
                            "{}\n{}",
                            rgb_color.bold().paint(github_sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else {
                        eprintln!("{}", rgb_color.bold().paint(message));
                    }
                }
            }
            PrintType::TimeMeasurement => {
                if pt.is_prints_enabled() && enable_provider_time_measurement {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.time_measurement_red,
                        CONFIG.time_measurement_green,
                        CONFIG.time_measurement_blue,
                    );
                    if CONFIG.is_show_source_place_enabled
                        && CONFIG.is_show_github_source_place_enabled
                    {
                        eprintln!(
                            "{}{}\n{}",
                            rgb_color.bold().paint(sources_track),
                            rgb_color.bold().paint(github_sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else if CONFIG.is_show_source_place_enabled {
                        eprintln!(
                            "{}\n{}",
                            rgb_color.bold().paint(sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else if CONFIG.is_show_github_source_place_enabled {
                        eprintln!(
                            "{}\n{}",
                            rgb_color.bold().paint(github_sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else {
                        eprintln!("{}", rgb_color.bold().paint(message));
                    }
                }
            }
            PrintType::CleaningWarningLogsDirectory => {
                if pt.is_prints_enabled()
                    && is_cleaning_warning_logs_directory_enabled_provider
                {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.cleaning_red,
                        CONFIG.cleaning_green,
                        CONFIG.cleaning_blue,
                    );
                    if CONFIG.is_show_source_place_enabled
                        && CONFIG.is_show_github_source_place_enabled
                    {
                        eprintln!(
                            "{}{}\n{}",
                            rgb_color.bold().paint(sources_track),
                            rgb_color.bold().paint(github_sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else if CONFIG.is_show_source_place_enabled {
                        eprintln!(
                            "{}\n{}",
                            rgb_color.bold().paint(sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else if CONFIG.is_show_github_source_place_enabled {
                        eprintln!(
                            "{}\n{}",
                            rgb_color.bold().paint(github_sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else {
                        eprintln!("{}", rgb_color.bold().paint(message));
                    }
                }
            }
            PrintType::Info => {
                if pt.is_prints_enabled() && is_info_prints_enabled_provider {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.info_red,
                        CONFIG.info_green,
                        CONFIG.info_blue,
                    );
                    if CONFIG.is_show_source_place_enabled
                        && CONFIG.is_show_github_source_place_enabled
                    {
                        println!(
                            "{}{}\n{}",
                            rgb_color.bold().paint(sources_track),
                            rgb_color.bold().paint(github_sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else if CONFIG.is_show_source_place_enabled {
                        println!(
                            "{}\n{}",
                            rgb_color.bold().paint(sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else if CONFIG.is_show_github_source_place_enabled {
                        println!(
                            "{}\n{}",
                            rgb_color.bold().paint(github_sources_track),
                            rgb_color.bold().paint(message)
                        );
                    } else {
                        println!("{}", rgb_color.bold().paint(message));
                    }
                }
            }
        }
    }
}