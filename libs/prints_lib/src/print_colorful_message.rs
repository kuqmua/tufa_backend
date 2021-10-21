use ansi_term::Colour::RGB;
use config_lib::get_project_information::get_config::get_lazy_config_information::CONFIG;
use config_lib::get_project_information::provider_kind_enum::ProviderKind;

use crate::print_type_enum::PrintType;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn print_colorful_message(
    provider_kind: Option<&ProviderKind>,
    print_type: PrintType,
    file: String,
    line: String,
    message: String,
) {
    if CONFIG.params.enable_prints {
        match provider_kind {
            Some(provider_kind) => {
                if CONFIG.params.enable_all_providers_prints {
                    match provider_kind {
                        ProviderKind::Arxiv => {
                            handle_provider_prints(
                                CONFIG.enable_providers_prints.enable_prints_arxiv,
                                CONFIG.params.enable_error_prints_for_all_providers,
                                CONFIG.enable_error_providers_prints.enable_error_prints_for_arxiv,
                                CONFIG.enable_warning_high_providers_prints.enable_warning_high_prints_for_arxiv,
                                CONFIG.enable_warning_low_providers_prints.enable_warning_low_prints_for_arxiv,
                                CONFIG.enable_success_providers_prints.enable_success_prints_for_arxiv,
                                CONFIG.enable_partial_success_providers_prints.enable_partial_success_prints_for_arxiv,
                                CONFIG.enable_providers_time_measurement.enable_time_measurement_for_arxiv,
                                CONFIG.enable_providers_cleaning_warning_logs_directory.enable_cleaning_warning_logs_directory_for_arxiv,
                                CONFIG.enable_providers_info.enable_info_for_arxiv,
                                print_type,
                                file,
                                line,
                                message,
                            );
                        }
                        ProviderKind::Biorxiv => {
                            handle_provider_prints(
                                CONFIG.enable_providers_prints.enable_prints_biorxiv,
                                CONFIG.params.enable_error_prints_for_all_providers,
                                CONFIG.enable_error_providers_prints.enable_error_prints_for_biorxiv,
                                CONFIG.enable_warning_high_providers_prints.enable_warning_high_prints_for_biorxiv,
                                CONFIG.enable_warning_low_providers_prints.enable_warning_low_prints_for_biorxiv,
                                CONFIG.enable_success_providers_prints.enable_success_prints_for_biorxiv,
                                CONFIG.enable_partial_success_providers_prints.enable_partial_success_prints_for_biorxiv,
                                CONFIG.enable_providers_time_measurement.enable_time_measurement_for_biorxiv,
                                CONFIG.enable_providers_cleaning_warning_logs_directory.enable_cleaning_warning_logs_directory_for_biorxiv,
                                CONFIG.enable_providers_info.enable_info_for_biorxiv,
                                print_type,
                                file,
                                line,
                                message,
                            );
                        }
                        ProviderKind::Github => {
                            handle_provider_prints(
                                CONFIG.enable_providers_prints.enable_prints_github,
                                CONFIG.params.enable_error_prints_for_all_providers,
                                CONFIG.enable_error_providers_prints.enable_error_prints_for_github,
                                CONFIG.enable_warning_high_providers_prints.enable_warning_high_prints_for_github,
                                CONFIG.enable_warning_low_providers_prints.enable_warning_low_prints_for_github,
                                CONFIG.enable_success_providers_prints.enable_success_prints_for_github,
                                CONFIG.enable_partial_success_providers_prints.enable_partial_success_prints_for_github,
                                CONFIG.enable_providers_time_measurement.enable_time_measurement_for_github,
                                CONFIG.enable_providers_cleaning_warning_logs_directory.enable_cleaning_warning_logs_directory_for_github,
                                CONFIG.enable_providers_info.enable_info_for_github,
                                print_type,
                                file,
                                line,
                                message,
                            );
                        }
                        ProviderKind::Habr => {
                            handle_provider_prints(
                                CONFIG.enable_providers_prints.enable_prints_habr,
                                CONFIG.params.enable_error_prints_for_all_providers,
                                CONFIG.enable_error_providers_prints.enable_error_prints_for_habr,
                                CONFIG.enable_warning_high_providers_prints.enable_warning_high_prints_for_habr,
                                CONFIG.enable_warning_low_providers_prints.enable_warning_low_prints_for_habr,
                                CONFIG.enable_success_providers_prints.enable_success_prints_for_habr,
                                CONFIG.enable_partial_success_providers_prints.enable_partial_success_prints_for_habr,
                                CONFIG.enable_providers_time_measurement.enable_time_measurement_for_habr,
                                CONFIG.enable_providers_cleaning_warning_logs_directory.enable_cleaning_warning_logs_directory_for_habr,
                                CONFIG.enable_providers_info.enable_info_for_habr,
                                print_type,
                                file,
                                line,
                                message,
                            );
                        }
                        ProviderKind::Medrxiv => {
                            handle_provider_prints(
                                CONFIG.enable_providers_prints.enable_prints_medrxiv,
                                CONFIG.params.enable_error_prints_for_all_providers,
                                CONFIG.enable_error_providers_prints.enable_error_prints_for_medrxiv,
                                CONFIG.enable_warning_high_providers_prints.enable_warning_high_prints_for_medrxiv,
                                CONFIG.enable_warning_low_providers_prints.enable_warning_low_prints_for_medrxiv,
                                CONFIG.enable_success_providers_prints.enable_success_prints_for_medrxiv,
                                CONFIG.enable_partial_success_providers_prints.enable_partial_success_prints_for_medrxiv,
                                CONFIG.enable_providers_time_measurement.enable_time_measurement_for_medrxiv,
                                CONFIG.enable_providers_cleaning_warning_logs_directory.enable_cleaning_warning_logs_directory_for_medrxiv,
                                CONFIG.enable_providers_info.enable_info_for_medrxiv,
                                print_type,
                                file,
                                line,
                                message,
                            );
                        }
                        ProviderKind::Reddit => {
                            handle_provider_prints(
                                CONFIG.enable_providers_prints.enable_prints_reddit,
                                CONFIG.params.enable_error_prints_for_all_providers,
                                CONFIG.enable_error_providers_prints.enable_error_prints_for_reddit,
                                CONFIG.enable_warning_high_providers_prints.enable_warning_high_prints_for_reddit,
                                CONFIG.enable_warning_low_providers_prints.enable_warning_low_prints_for_reddit,
                                CONFIG.enable_success_providers_prints.enable_success_prints_for_reddit,
                                CONFIG.enable_partial_success_providers_prints.enable_partial_success_prints_for_reddit,
                                CONFIG.enable_providers_time_measurement.enable_time_measurement_for_reddit,
                                CONFIG.enable_providers_cleaning_warning_logs_directory.enable_cleaning_warning_logs_directory_for_reddit,
                                CONFIG.enable_providers_info.enable_info_for_reddit,
                                print_type,
                                file,
                                line,
                                message,
                            );
                        }
                        ProviderKind::Twitter => {
                            handle_provider_prints(
                                CONFIG.enable_providers_prints.enable_prints_twitter,
                                CONFIG.params.enable_error_prints_for_all_providers,
                                CONFIG.enable_error_providers_prints.enable_error_prints_for_twitter,
                                CONFIG.enable_warning_high_providers_prints.enable_warning_high_prints_for_twitter,
                                CONFIG.enable_warning_low_providers_prints.enable_warning_low_prints_for_twitter,
                                CONFIG.enable_success_providers_prints.enable_success_prints_for_twitter,
                                CONFIG.enable_partial_success_providers_prints.enable_partial_success_prints_for_twitter,
                                CONFIG.enable_providers_time_measurement.enable_time_measurement_for_twitter,
                                CONFIG.enable_providers_cleaning_warning_logs_directory.enable_cleaning_warning_logs_directory_for_twitter,
                                CONFIG.enable_providers_info.enable_info_for_twitter,
                                print_type,
                                file,
                                line,
                                message,
                            );
                        }
                    }
                }
            }
            None => match print_type {
                PrintType::Error => {
                    if CONFIG.params.enable_error_prints {
                        let rgb_color: ansi_term::Colour = RGB(
                            CONFIG.print_colors.error_red,
                            CONFIG.print_colors.error_green,
                            CONFIG.print_colors.error_blue,
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
                PrintType::WarningHigh => {
                    if CONFIG.params.enable_warning_high_prints {
                        let rgb_color: ansi_term::Colour = RGB(
                            CONFIG.print_colors.warning_high_red,
                            CONFIG.print_colors.warning_high_green,
                            CONFIG.print_colors.warning_high_blue,
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
                    if CONFIG.params.enable_warning_low_prints {
                        let rgb_color: ansi_term::Colour = RGB(
                            CONFIG.print_colors.warning_low_red,
                            CONFIG.print_colors.warning_low_green,
                            CONFIG.print_colors.warning_low_blue,
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
                    if CONFIG.params.enable_success_prints {
                        let rgb_color: ansi_term::Colour = RGB(
                            CONFIG.print_colors.success_red,
                            CONFIG.print_colors.success_green,
                            CONFIG.print_colors.success_blue,
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
                    if CONFIG.params.enable_partial_success_prints {
                        let rgb_color: ansi_term::Colour = RGB(
                            CONFIG.print_colors.partial_success_red,
                            CONFIG.print_colors.partial_success_green,
                            CONFIG.print_colors.partial_success_blue,
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
                    if CONFIG.params.enable_time_measurement_prints {
                        let rgb_color: ansi_term::Colour = RGB(
                            CONFIG.print_colors.time_measurement_red,
                            CONFIG.print_colors.time_measurement_green,
                            CONFIG.print_colors.time_measurement_blue,
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
                    if CONFIG.params.enable_cleaning_warning_logs_directory_prints {
                        let rgb_color: ansi_term::Colour = RGB(
                            CONFIG.print_colors.cleaning_red,
                            CONFIG.print_colors.cleaning_green,
                            CONFIG.print_colors.cleaning_blue,
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
                    if CONFIG.params.enable_info_prints {
                        let rgb_color: ansi_term::Colour = RGB(
                            CONFIG.print_colors.info_red,
                            CONFIG.print_colors.info_green,
                            CONFIG.print_colors.info_blue,
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
            },
        }
    }
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[allow(clippy::too_many_arguments)]
fn handle_provider_prints(
    enable_prints_provider: bool,
    enable_error_prints_for_all_providers: bool,
    enable_error_prints_for_provider: bool,
    enable_warning_high_prints_for_provider: bool,
    enable_warning_low_prints_for_provider: bool,
    enable_success_prints_for_provider: bool,
    enable_partial_success_prints_for_provider: bool,
    enable_provider_time_measurement: bool,
    enable_cleaning_warning_logs_directory_for_provider: bool,
    enable_info_prints_for_provider: bool,
    print_type: PrintType,
    file: String,
    line: String,
    message: String,
) {
    if enable_prints_provider {
        match print_type {
            PrintType::Error => {
                if CONFIG.params.enable_error_prints
                    && enable_error_prints_for_all_providers
                    && enable_error_prints_for_provider
                {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.print_colors.error_red,
                        CONFIG.print_colors.error_green,
                        CONFIG.print_colors.error_blue,
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
            PrintType::WarningHigh => {
                if CONFIG.params.enable_warning_high_prints
                    && enable_warning_high_prints_for_provider
                {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.print_colors.warning_high_red,
                        CONFIG.print_colors.warning_high_green,
                        CONFIG.print_colors.warning_high_blue,
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
                if CONFIG.params.enable_warning_low_prints
                    && enable_warning_low_prints_for_provider
                {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.print_colors.warning_low_red,
                        CONFIG.print_colors.warning_low_green,
                        CONFIG.print_colors.warning_low_blue,
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
                if CONFIG.params.enable_success_prints
                    && enable_success_prints_for_provider
                {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.print_colors.success_red,
                        CONFIG.print_colors.success_green,
                        CONFIG.print_colors.success_blue,
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
                if CONFIG.params.enable_partial_success_prints
                    && enable_partial_success_prints_for_provider
                {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.print_colors.partial_success_red,
                        CONFIG.print_colors.partial_success_green,
                        CONFIG.print_colors.partial_success_blue,
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
                if CONFIG.params.enable_time_measurement_prints
                    && enable_provider_time_measurement
                {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.print_colors.time_measurement_red,
                        CONFIG.print_colors.time_measurement_green,
                        CONFIG.print_colors.time_measurement_blue,
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
                if CONFIG.params.enable_cleaning_warning_logs_directory_prints
                    && enable_cleaning_warning_logs_directory_for_provider
                {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.print_colors.cleaning_red,
                        CONFIG.print_colors.cleaning_green,
                        CONFIG.print_colors.cleaning_blue,
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
                if CONFIG.params.enable_info_prints
                    && enable_info_prints_for_provider
                {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.print_colors.cleaning_red,
                        CONFIG.print_colors.cleaning_green,
                        CONFIG.print_colors.cleaning_blue,
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
