use crate::config_mods::lazy_static_config::CONFIG;
use crate::providers::provider_kind_enum::ProviderKind;
use ansi_term::Colour::RGB;

use crate::prints::print_type_enum::PrintType;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn print_colorful_message(
    provider_kind: Option<&ProviderKind>,
    print_type: PrintType,
    file: String,
    line: String,
    message: String,
) {
    if CONFIG.enable_prints {
        match provider_kind {
            Some(provider_kind) => match provider_kind {
                ProviderKind::Arxiv => {
                    handle_provider_prints(
                        CONFIG.is_prints_enabled_arxiv,
                        CONFIG.enable_error_prints_for_arxiv,
                        CONFIG.is_warning_high_prints_enabled_arxiv,
                        CONFIG.enable_warning_low_prints_for_arxiv,
                        CONFIG.enable_success_prints_for_arxiv,
                        CONFIG.enable_partial_success_prints_for_arxiv,
                        CONFIG.enable_time_measurement_prints_for_arxiv,
                        CONFIG.is_cleaning_warning_logs_directory_enabled_arxiv,
                        CONFIG.enable_info_prints_for_arxiv,
                        print_type,
                        file,
                        line,
                        message,
                    );
                }
                ProviderKind::Biorxiv => {
                    handle_provider_prints(
                        CONFIG.is_prints_enabled_biorxiv,
                        CONFIG.enable_error_prints_for_biorxiv,
                        CONFIG.is_warning_high_prints_enabled_biorxiv,
                        CONFIG.enable_warning_low_prints_for_biorxiv,
                        CONFIG.enable_success_prints_for_biorxiv,
                        CONFIG.enable_partial_success_prints_for_biorxiv,
                        CONFIG.enable_time_measurement_prints_for_biorxiv,
                        CONFIG.is_cleaning_warning_logs_directory_enabled_biorxiv,
                        CONFIG.enable_info_prints_for_biorxiv,
                        print_type,
                        file,
                        line,
                        message,
                    );
                }
                ProviderKind::Github => {
                    handle_provider_prints(
                        CONFIG.is_prints_enabled_github,
                        CONFIG.enable_error_prints_for_github,
                        CONFIG.is_warning_high_prints_enabled_github,
                        CONFIG.enable_warning_low_prints_for_github,
                        CONFIG.enable_success_prints_for_github,
                        CONFIG.enable_partial_success_prints_for_github,
                        CONFIG.enable_time_measurement_prints_for_github,
                        CONFIG.is_cleaning_warning_logs_directory_enabled_github,
                        CONFIG.enable_info_prints_for_github,
                        print_type,
                        file,
                        line,
                        message,
                    );
                }
                ProviderKind::Habr => {
                    handle_provider_prints(
                        CONFIG.is_prints_enabled_habr,
                        CONFIG.enable_error_prints_for_habr,
                        CONFIG.is_warning_high_prints_enabled_habr,
                        CONFIG.enable_warning_low_prints_for_habr,
                        CONFIG.enable_success_prints_for_habr,
                        CONFIG.enable_partial_success_prints_for_habr,
                        CONFIG.enable_time_measurement_prints_for_habr,
                        CONFIG.is_cleaning_warning_logs_directory_enabled_habr,
                        CONFIG.enable_info_prints_for_habr,
                        print_type,
                        file,
                        line,
                        message,
                    );
                }
                ProviderKind::Medrxiv => {
                    handle_provider_prints(
                        CONFIG.is_prints_enabled_medrxiv,
                        CONFIG.enable_error_prints_for_medrxiv,
                        CONFIG.is_warning_high_prints_enabled_medrxiv,
                        CONFIG.enable_warning_low_prints_for_medrxiv,
                        CONFIG.enable_success_prints_for_medrxiv,
                        CONFIG.enable_partial_success_prints_for_medrxiv,
                        CONFIG.enable_time_measurement_prints_for_medrxiv,
                        CONFIG.is_cleaning_warning_logs_directory_enabled_medrxiv,
                        CONFIG.enable_info_prints_for_medrxiv,
                        print_type,
                        file,
                        line,
                        message,
                    );
                }
                ProviderKind::Reddit => {
                    handle_provider_prints(
                        CONFIG.is_prints_enabled_reddit,
                        CONFIG.enable_error_prints_for_reddit,
                        CONFIG.is_warning_high_prints_enabled_reddit,
                        CONFIG.enable_warning_low_prints_for_reddit,
                        CONFIG.enable_success_prints_for_reddit,
                        CONFIG.enable_partial_success_prints_for_reddit,
                        CONFIG.enable_time_measurement_prints_for_reddit,
                        CONFIG.is_cleaning_warning_logs_directory_enabled_reddit,
                        CONFIG.enable_info_prints_for_reddit,
                        print_type,
                        file,
                        line,
                        message,
                    );
                }
                ProviderKind::Twitter => {
                    handle_provider_prints(
                        CONFIG.is_prints_enabled_twitter,
                        CONFIG.enable_error_prints_for_twitter,
                        CONFIG.is_warning_high_prints_enabled_twitter,
                        CONFIG.enable_warning_low_prints_for_twitter,
                        CONFIG.enable_success_prints_for_twitter,
                        CONFIG.enable_partial_success_prints_for_twitter,
                        CONFIG.enable_time_measurement_prints_for_twitter,
                        CONFIG.is_cleaning_warning_logs_directory_enabled_twitter,
                        CONFIG.enable_info_prints_for_twitter,
                        print_type,
                        file,
                        line,
                        message,
                    );
                }
            },
            None => match print_type {
                PrintType::Error => {
                    if CONFIG.enable_error_prints {
                        let rgb_color: ansi_term::Colour =
                            RGB(CONFIG.error_red, CONFIG.error_green, CONFIG.error_blue);
                        eprintln!(
                            "{}{}{}{}\n{}",
                            rgb_color.paint("file: "),
                            rgb_color.paint(file),
                            rgb_color.paint(":"),
                            rgb_color.paint(line),
                            rgb_color.bold().paint(message)
                        );
                    }
                }
                PrintType::WarningHigh => {
                    if CONFIG.is_warning_high_prints_enabled {
                        let rgb_color: ansi_term::Colour = RGB(
                            CONFIG.warning_high_red,
                            CONFIG.warning_high_green,
                            CONFIG.warning_high_blue,
                        );
                        eprintln!(
                            "{}{}{}{}\n{}",
                            rgb_color.paint("file: "),
                            rgb_color.paint(file),
                            rgb_color.paint(":"),
                            rgb_color.paint(line),
                            rgb_color.bold().paint(message)
                        );
                    }
                }
                PrintType::WarningLow => {
                    if CONFIG.enable_warning_low_prints {
                        let rgb_color: ansi_term::Colour = RGB(
                            CONFIG.warning_low_red,
                            CONFIG.warning_low_green,
                            CONFIG.warning_low_blue,
                        );
                        eprintln!(
                            "{}{}{}{}\n{}",
                            rgb_color.paint("file: "),
                            rgb_color.paint(file),
                            rgb_color.paint(":"),
                            rgb_color.paint(line),
                            rgb_color.bold().paint(message)
                        );
                    }
                }
                PrintType::Success => {
                    if CONFIG.enable_success_prints {
                        let rgb_color: ansi_term::Colour = RGB(
                            CONFIG.success_red,
                            CONFIG.success_green,
                            CONFIG.success_blue,
                        );
                        eprintln!(
                            "{}{}{}{}\n{}",
                            rgb_color.paint("file: "),
                            rgb_color.paint(file),
                            rgb_color.paint(":"),
                            rgb_color.paint(line),
                            rgb_color.bold().paint(message)
                        );
                    }
                }
                PrintType::PartialSuccess => {
                    if CONFIG.enable_partial_success_prints {
                        let rgb_color: ansi_term::Colour = RGB(
                            CONFIG.partial_success_red,
                            CONFIG.partial_success_green,
                            CONFIG.partial_success_blue,
                        );
                        eprintln!(
                            "{}{}{}{}\n{}",
                            rgb_color.paint("file: "),
                            rgb_color.paint(file),
                            rgb_color.paint(":"),
                            rgb_color.paint(line),
                            rgb_color.bold().paint(message)
                        );
                    }
                }
                PrintType::TimeMeasurement => {
                    if CONFIG.enable_time_measurement_prints {
                        let rgb_color: ansi_term::Colour = RGB(
                            CONFIG.time_measurement_red,
                            CONFIG.time_measurement_green,
                            CONFIG.time_measurement_blue,
                        );
                        eprintln!(
                            "{}{}{}{}\n{}",
                            rgb_color.paint("file: "),
                            rgb_color.paint(file),
                            rgb_color.paint(":"),
                            rgb_color.paint(line),
                            rgb_color.bold().paint(message)
                        );
                    }
                }
                PrintType::CleaningWarningLogsDirectory => {
                    if CONFIG.is_cleaning_warning_logs_directory_enabled {
                        let rgb_color: ansi_term::Colour = RGB(
                            CONFIG.cleaning_red,
                            CONFIG.cleaning_green,
                            CONFIG.cleaning_blue,
                        );
                        eprintln!(
                            "{}{}{}{}\n{}",
                            rgb_color.paint("file: "),
                            rgb_color.paint(file),
                            rgb_color.paint(":"),
                            rgb_color.paint(line),
                            rgb_color.bold().paint(message)
                        );
                    }
                }
                PrintType::Info => {
                    if CONFIG.enable_info_prints {
                        let rgb_color: ansi_term::Colour =
                            RGB(CONFIG.info_red, CONFIG.info_green, CONFIG.info_blue);
                        eprintln!(
                            "{}{}{}{}\n{}",
                            rgb_color.paint("file: "),
                            rgb_color.paint(file),
                            rgb_color.paint(":"),
                            rgb_color.paint(line),
                            rgb_color.bold().paint(message)
                        );
                    }
                }
            },
        }
    }
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[allow(clippy::too_many_arguments)]
fn handle_provider_prints(
    is_prints_enabled_provider: bool,
    enable_error_prints_for_provider: bool,
    is_warning_high_prints_enabled_provider: bool,
    enable_warning_low_prints_for_provider: bool,
    enable_success_prints_for_provider: bool,
    enable_partial_success_prints_for_provider: bool,
    enable_provider_time_measurement: bool,
    is_cleaning_warning_logs_directory_enabled_provider: bool,
    enable_info_prints_for_provider: bool,
    print_type: PrintType,
    file: String,
    line: String,
    message: String,
) {
    if is_prints_enabled_provider {
        match print_type {
            PrintType::Error => {
                if CONFIG.enable_error_prints && enable_error_prints_for_provider {
                    let rgb_color: ansi_term::Colour =
                        RGB(CONFIG.error_red, CONFIG.error_green, CONFIG.error_blue);
                    eprintln!(
                        "{}{}{}{}\n{}",
                        rgb_color.paint("file: "),
                        rgb_color.paint(file),
                        rgb_color.paint(":"),
                        rgb_color.paint(line),
                        rgb_color.bold().paint(message)
                    );
                }
            }
            PrintType::WarningHigh => {
                if CONFIG.is_warning_high_prints_enabled && is_warning_high_prints_enabled_provider {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.warning_high_red,
                        CONFIG.warning_high_green,
                        CONFIG.warning_high_blue,
                    );
                    eprintln!(
                        "{}{}{}{}\n{}",
                        rgb_color.paint("file: "),
                        rgb_color.paint(file),
                        rgb_color.paint(":"),
                        rgb_color.paint(line),
                        rgb_color.bold().paint(message)
                    );
                }
            }
            PrintType::WarningLow => {
                if CONFIG.enable_warning_low_prints && enable_warning_low_prints_for_provider {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.warning_low_red,
                        CONFIG.warning_low_green,
                        CONFIG.warning_low_blue,
                    );
                    eprintln!(
                        "{}{}{}{}\n{}",
                        rgb_color.paint("file: "),
                        rgb_color.paint(file),
                        rgb_color.paint(":"),
                        rgb_color.paint(line),
                        rgb_color.bold().paint(message)
                    );
                }
            }
            PrintType::Success => {
                if CONFIG.enable_success_prints && enable_success_prints_for_provider {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.success_red,
                        CONFIG.success_green,
                        CONFIG.success_blue,
                    );
                    eprintln!(
                        "{}{}{}{}\n{}",
                        rgb_color.paint("file: "),
                        rgb_color.paint(file),
                        rgb_color.paint(":"),
                        rgb_color.paint(line),
                        rgb_color.bold().paint(message)
                    );
                }
            }
            PrintType::PartialSuccess => {
                if CONFIG.enable_partial_success_prints
                    && enable_partial_success_prints_for_provider
                {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.partial_success_red,
                        CONFIG.partial_success_green,
                        CONFIG.partial_success_blue,
                    );
                    eprintln!(
                        "{}{}{}{}\n{}",
                        rgb_color.paint("file: "),
                        rgb_color.paint(file),
                        rgb_color.paint(":"),
                        rgb_color.paint(line),
                        rgb_color.bold().paint(message)
                    );
                }
            }
            PrintType::TimeMeasurement => {
                if CONFIG.enable_time_measurement_prints && enable_provider_time_measurement {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.time_measurement_red,
                        CONFIG.time_measurement_green,
                        CONFIG.time_measurement_blue,
                    );
                    eprintln!(
                        "{}{}{}{}\n{}",
                        rgb_color.paint("file: "),
                        rgb_color.paint(file),
                        rgb_color.paint(":"),
                        rgb_color.paint(line),
                        rgb_color.bold().paint(message)
                    );
                }
            }
            PrintType::CleaningWarningLogsDirectory => {
                if CONFIG.is_cleaning_warning_logs_directory_enabled
                    && is_cleaning_warning_logs_directory_enabled_provider
                {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.cleaning_red,
                        CONFIG.cleaning_green,
                        CONFIG.cleaning_blue,
                    );
                    eprintln!(
                        "{}{}{}{}\n{}",
                        rgb_color.paint("file: "),
                        rgb_color.paint(file),
                        rgb_color.paint(":"),
                        rgb_color.paint(line),
                        rgb_color.bold().paint(message)
                    );
                }
            }
            PrintType::Info => {
                if CONFIG.enable_info_prints && enable_info_prints_for_provider {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.cleaning_red,
                        CONFIG.cleaning_green,
                        CONFIG.cleaning_blue,
                    );
                    eprintln!(
                        "{}{}{}{}\n{}",
                        rgb_color.paint("file: "),
                        rgb_color.paint(file),
                        rgb_color.paint(":"),
                        rgb_color.paint(line),
                        rgb_color.bold().paint(message)
                    );
                }
            }
        }
    }
}
