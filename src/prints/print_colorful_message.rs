use crate::config_mods::lazy_static_config::CONFIG;
use crate::providers::provider_kind_enum::ProviderKind;
use ansi_term::Colour::RGB;

use crate::prints::print_type_enum::PrintType;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn print_colorful_message(
    pk: Option<&ProviderKind>,
    print_type: PrintType,
    sources: Vec<String>,
    github_sources: Vec<String>,
    message: String,
) {
    let mut sources_track = String::from("");
    for source in sources {
        sources_track.push_str(&format!("{}\n", source))
    }
    let mut github_sources_track = String::from("");
    for github_source in github_sources {
        github_sources_track.push_str(&format!("{}\n", github_source))
    }
    if CONFIG.is_prints_enabled {
        match pk {
            Some(pk) => match pk {
                ProviderKind::Arxiv => {
                    handle_provider_prints(
                        CONFIG.is_prints_enabled_arxiv,
                        CONFIG.is_error_prints_enabled_arxiv,
                        CONFIG.is_warning_high_prints_enabled_arxiv,
                        CONFIG.is_warning_low_prints_enabled_arxiv,
                        CONFIG.is_success_prints_enabled_arxiv,
                        CONFIG.is_partial_success_prints_enabled_arxiv,
                        CONFIG.is_time_measurement_prints_enabled_arxiv,
                        CONFIG.is_cleaning_warning_logs_directory_enabled_arxiv,
                        CONFIG.is_info_prints_enabled_arxiv,
                        print_type,
                        sources_track,
                        github_sources_track,
                        message,
                    );
                }
                ProviderKind::Biorxiv => {
                    handle_provider_prints(
                        CONFIG.is_prints_enabled_biorxiv,
                        CONFIG.is_error_prints_enabled_biorxiv,
                        CONFIG.is_warning_high_prints_enabled_biorxiv,
                        CONFIG.is_warning_low_prints_enabled_biorxiv,
                        CONFIG.is_success_prints_enabled_biorxiv,
                        CONFIG.is_partial_success_prints_enabled_biorxiv,
                        CONFIG.is_time_measurement_prints_enabled_biorxiv,
                        CONFIG.is_cleaning_warning_logs_directory_enabled_biorxiv,
                        CONFIG.is_info_prints_enabled_biorxiv,
                        print_type,
                        sources_track,
                        github_sources_track,
                        message,
                    );
                }
                ProviderKind::Github => {
                    handle_provider_prints(
                        CONFIG.is_prints_enabled_github,
                        CONFIG.is_error_prints_enabled_github,
                        CONFIG.is_warning_high_prints_enabled_github,
                        CONFIG.is_warning_low_prints_enabled_github,
                        CONFIG.is_success_prints_enabled_github,
                        CONFIG.is_partial_success_prints_enabled_github,
                        CONFIG.is_time_measurement_prints_enabled_github,
                        CONFIG.is_cleaning_warning_logs_directory_enabled_github,
                        CONFIG.is_info_prints_enabled_github,
                        print_type,
                        sources_track,
                        github_sources_track,
                        message,
                    );
                }
                ProviderKind::Habr => {
                    handle_provider_prints(
                        CONFIG.is_prints_enabled_habr,
                        CONFIG.is_error_prints_enabled_habr,
                        CONFIG.is_warning_high_prints_enabled_habr,
                        CONFIG.is_warning_low_prints_enabled_habr,
                        CONFIG.is_success_prints_enabled_habr,
                        CONFIG.is_partial_success_prints_enabled_habr,
                        CONFIG.is_time_measurement_prints_enabled_habr,
                        CONFIG.is_cleaning_warning_logs_directory_enabled_habr,
                        CONFIG.is_info_prints_enabled_habr,
                        print_type,
                        sources_track,
                        github_sources_track,
                        message,
                    );
                }
                ProviderKind::Medrxiv => {
                    handle_provider_prints(
                        CONFIG.is_prints_enabled_medrxiv,
                        CONFIG.is_error_prints_enabled_medrxiv,
                        CONFIG.is_warning_high_prints_enabled_medrxiv,
                        CONFIG.is_warning_low_prints_enabled_medrxiv,
                        CONFIG.is_success_prints_enabled_medrxiv,
                        CONFIG.is_partial_success_prints_enabled_medrxiv,
                        CONFIG.is_time_measurement_prints_enabled_medrxiv,
                        CONFIG.is_cleaning_warning_logs_directory_enabled_medrxiv,
                        CONFIG.is_info_prints_enabled_medrxiv,
                        print_type,
                        sources_track,
                        github_sources_track,
                        message,
                    );
                }
                ProviderKind::Reddit => {
                    handle_provider_prints(
                        CONFIG.is_prints_enabled_reddit,
                        CONFIG.is_error_prints_enabled_reddit,
                        CONFIG.is_warning_high_prints_enabled_reddit,
                        CONFIG.is_warning_low_prints_enabled_reddit,
                        CONFIG.is_success_prints_enabled_reddit,
                        CONFIG.is_partial_success_prints_enabled_reddit,
                        CONFIG.is_time_measurement_prints_enabled_reddit,
                        CONFIG.is_cleaning_warning_logs_directory_enabled_reddit,
                        CONFIG.is_info_prints_enabled_reddit,
                        print_type,
                        sources_track,
                        github_sources_track,
                        message,
                    );
                }
                ProviderKind::Twitter => {
                    handle_provider_prints(
                        CONFIG.is_prints_enabled_twitter,
                        CONFIG.is_error_prints_enabled_twitter,
                        CONFIG.is_warning_high_prints_enabled_twitter,
                        CONFIG.is_warning_low_prints_enabled_twitter,
                        CONFIG.is_success_prints_enabled_twitter,
                        CONFIG.is_partial_success_prints_enabled_twitter,
                        CONFIG.is_time_measurement_prints_enabled_twitter,
                        CONFIG.is_cleaning_warning_logs_directory_enabled_twitter,
                        CONFIG.is_info_prints_enabled_twitter,
                        print_type,
                        sources_track,
                        github_sources_track,
                        message,
                    );
                }
            },
            None => match print_type {
                PrintType::Error => {
                    if CONFIG.is_error_prints_enabled {
                        let rgb_color: ansi_term::Colour =
                            RGB(CONFIG.error_red, CONFIG.error_green, CONFIG.error_blue);
                        if CONFIG.is_show_github_source_place_enabled {
                            eprintln!(
                                "{}{}\n{}",
                                sources_track,
                                github_sources_track,
                                rgb_color.bold().paint(message)
                            );
                        } else {
                            eprintln!("{}\n{}", sources_track, rgb_color.bold().paint(message));
                        }
                    }
                }
                PrintType::WarningHigh => {
                    if CONFIG.is_warning_high_prints_enabled {
                        let rgb_color: ansi_term::Colour = RGB(
                            CONFIG.warning_high_red,
                            CONFIG.warning_high_green,
                            CONFIG.warning_high_blue,
                        );
                        if CONFIG.is_show_github_source_place_enabled {
                            eprintln!(
                                "{}{}\n{}",
                                sources_track,
                                github_sources_track,
                                rgb_color.bold().paint(message)
                            );
                        } else {
                            eprintln!("{}\n{}", sources_track, rgb_color.bold().paint(message));
                        }
                    }
                }
                PrintType::WarningLow => {
                    if CONFIG.is_warning_low_prints_enabled {
                        let rgb_color: ansi_term::Colour = RGB(
                            CONFIG.warning_low_red,
                            CONFIG.warning_low_green,
                            CONFIG.warning_low_blue,
                        );
                        if CONFIG.is_show_github_source_place_enabled {
                            eprintln!(
                                "{}{}\n{}",
                                sources_track,
                                github_sources_track,
                                rgb_color.bold().paint(message)
                            );
                        } else {
                            eprintln!("{}\n{}", sources_track, rgb_color.bold().paint(message));
                        }
                    }
                }
                PrintType::Success => {
                    if CONFIG.is_success_prints_enabled {
                        let rgb_color: ansi_term::Colour = RGB(
                            CONFIG.success_red,
                            CONFIG.success_green,
                            CONFIG.success_blue,
                        );
                        if CONFIG.is_show_github_source_place_enabled {
                            eprintln!(
                                "{}{}\n{}",
                                sources_track,
                                github_sources_track,
                                rgb_color.bold().paint(message)
                            );
                        } else {
                            eprintln!("{}\n{}", sources_track, rgb_color.bold().paint(message));
                        }
                    }
                }
                PrintType::PartialSuccess => {
                    if CONFIG.is_partial_success_prints_enabled {
                        let rgb_color: ansi_term::Colour = RGB(
                            CONFIG.partial_success_red,
                            CONFIG.partial_success_green,
                            CONFIG.partial_success_blue,
                        );
                        if CONFIG.is_show_github_source_place_enabled {
                            eprintln!(
                                "{}{}\n{}",
                                sources_track,
                                github_sources_track,
                                rgb_color.bold().paint(message)
                            );
                        } else {
                            eprintln!("{}\n{}", sources_track, rgb_color.bold().paint(message));
                        }
                    }
                }
                PrintType::TimeMeasurement => {
                    if CONFIG.is_time_measurement_prints_enabled {
                        let rgb_color: ansi_term::Colour = RGB(
                            CONFIG.time_measurement_red,
                            CONFIG.time_measurement_green,
                            CONFIG.time_measurement_blue,
                        );
                        if CONFIG.is_show_github_source_place_enabled {
                            eprintln!(
                                "{}{}\n{}",
                                sources_track,
                                github_sources_track,
                                rgb_color.bold().paint(message)
                            );
                        } else {
                            eprintln!("{}\n{}", sources_track, rgb_color.bold().paint(message));
                        }
                    }
                }
                PrintType::CleaningWarningLogsDirectory => {
                    if CONFIG.is_cleaning_warning_logs_directory_enabled {
                        let rgb_color: ansi_term::Colour = RGB(
                            CONFIG.cleaning_red,
                            CONFIG.cleaning_green,
                            CONFIG.cleaning_blue,
                        );
                        if CONFIG.is_show_github_source_place_enabled {
                            eprintln!(
                                "{}{}\n{}",
                                sources_track,
                                github_sources_track,
                                rgb_color.bold().paint(message)
                            );
                        } else {
                            eprintln!("{}\n{}", sources_track, rgb_color.bold().paint(message));
                        }
                    }
                }
                PrintType::Info => {
                    if CONFIG.is_info_prints_enabled {
                        let rgb_color: ansi_term::Colour =
                            RGB(CONFIG.info_red, CONFIG.info_green, CONFIG.info_blue);
                        if CONFIG.is_show_github_source_place_enabled {
                            println!(
                                "{}{}\n{}",
                                sources_track,
                                github_sources_track,
                                rgb_color.bold().paint(message)
                            );
                        } else {
                            println!("{}\n{}", sources_track, rgb_color.bold().paint(message));
                        }
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
    is_error_prints_enabled_provider: bool,
    is_warning_high_prints_enabled_provider: bool,
    is_warning_low_prints_enabled_provider: bool,
    is_success_prints_enabled_provider: bool,
    is_partial_success_prints_enabled_provider: bool,
    enable_provider_time_measurement: bool,
    is_cleaning_warning_logs_directory_enabled_provider: bool,
    is_info_prints_enabled_provider: bool,
    print_type: PrintType,
    sources_track: String,
    github_sources_track: String,
    message: String,
) {
    if is_prints_enabled_provider {
        match print_type {
            PrintType::Error => {
                if CONFIG.is_error_prints_enabled && is_error_prints_enabled_provider {
                    let rgb_color: ansi_term::Colour =
                        RGB(CONFIG.error_red, CONFIG.error_green, CONFIG.error_blue);
                    if CONFIG.is_show_github_source_place_enabled {
                        eprintln!(
                            "{}{}\n{}",
                            sources_track,
                            github_sources_track,
                            rgb_color.bold().paint(message)
                        );
                    } else {
                        eprintln!("{}\n{}", sources_track, rgb_color.bold().paint(message));
                    }
                }
            }
            PrintType::WarningHigh => {
                if CONFIG.is_warning_high_prints_enabled && is_warning_high_prints_enabled_provider
                {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.warning_high_red,
                        CONFIG.warning_high_green,
                        CONFIG.warning_high_blue,
                    );
                    if CONFIG.is_show_github_source_place_enabled {
                        eprintln!(
                            "{}{}\n{}",
                            sources_track,
                            github_sources_track,
                            rgb_color.bold().paint(message)
                        );
                    } else {
                        eprintln!("{}\n{}", sources_track, rgb_color.bold().paint(message));
                    }
                }
            }
            PrintType::WarningLow => {
                if CONFIG.is_warning_low_prints_enabled && is_warning_low_prints_enabled_provider {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.warning_low_red,
                        CONFIG.warning_low_green,
                        CONFIG.warning_low_blue,
                    );
                    if CONFIG.is_show_github_source_place_enabled {
                        eprintln!(
                            "{}{}\n{}",
                            sources_track,
                            github_sources_track,
                            rgb_color.bold().paint(message)
                        );
                    } else {
                        eprintln!("{}\n{}", sources_track, rgb_color.bold().paint(message));
                    }
                }
            }
            PrintType::Success => {
                if CONFIG.is_success_prints_enabled && is_success_prints_enabled_provider {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.success_red,
                        CONFIG.success_green,
                        CONFIG.success_blue,
                    );
                    if CONFIG.is_show_github_source_place_enabled {
                        eprintln!(
                            "{}{}\n{}",
                            sources_track,
                            github_sources_track,
                            rgb_color.bold().paint(message)
                        );
                    } else {
                        eprintln!("{}\n{}", sources_track, rgb_color.bold().paint(message));
                    }
                }
            }
            PrintType::PartialSuccess => {
                if CONFIG.is_partial_success_prints_enabled
                    && is_partial_success_prints_enabled_provider
                {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.partial_success_red,
                        CONFIG.partial_success_green,
                        CONFIG.partial_success_blue,
                    );
                    if CONFIG.is_show_github_source_place_enabled {
                        eprintln!(
                            "{}{}\n{}",
                            sources_track,
                            github_sources_track,
                            rgb_color.bold().paint(message)
                        );
                    } else {
                        eprintln!("{}\n{}", sources_track, rgb_color.bold().paint(message));
                    }
                }
            }
            PrintType::TimeMeasurement => {
                if CONFIG.is_time_measurement_prints_enabled && enable_provider_time_measurement {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.time_measurement_red,
                        CONFIG.time_measurement_green,
                        CONFIG.time_measurement_blue,
                    );
                    if CONFIG.is_show_github_source_place_enabled {
                        eprintln!(
                            "{}{}\n{}",
                            sources_track,
                            github_sources_track,
                            rgb_color.bold().paint(message)
                        );
                    } else {
                        eprintln!("{}\n{}", sources_track, rgb_color.bold().paint(message));
                    }
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
                    if CONFIG.is_show_github_source_place_enabled {
                        eprintln!(
                            "{}{}\n{}",
                            sources_track,
                            github_sources_track,
                            rgb_color.bold().paint(message)
                        );
                    } else {
                        eprintln!("{}\n{}", sources_track, rgb_color.bold().paint(message));
                    }
                }
            }
            PrintType::Info => {
                if CONFIG.is_info_prints_enabled && is_info_prints_enabled_provider {
                    let rgb_color: ansi_term::Colour = RGB(
                        CONFIG.cleaning_red,
                        CONFIG.cleaning_green,
                        CONFIG.cleaning_blue,
                    );
                    if CONFIG.is_show_github_source_place_enabled {
                        println!(
                            "{}{}\n{}",
                            sources_track,
                            github_sources_track,
                            rgb_color.bold().paint(message)
                        );
                    } else {
                        println!("{}\n{}", sources_track, rgb_color.bold().paint(message));
                    }
                }
            }
        }
    }
}
