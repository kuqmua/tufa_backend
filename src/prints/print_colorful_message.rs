use crate::config_mods::lazy_static_config::CONFIG;
use crate::providers::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_from_config_trait::ProviderKindFromConfigTrait;
use ansi_term::Colour::RGB;

use crate::prints::print_type_enum::PrintType;

use crate::prints::handle_provider_prints::handle_provider_prints;

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
            Some(pk) =>  {
                handle_provider_prints(
                    pk.is_prints_enabled(),
                    pk.is_error_prints_enabled(),
                    pk.is_warning_high_prints_enabled(),
                    pk.is_warning_low_prints_enabled(),
                    pk.is_success_prints_enabled(),
                    pk.is_partial_success_prints_enabled(),
                    pk.is_time_measurement_prints_enabled(),
                    pk.is_cleaning_warning_logs_directory_enabled(),
                    pk.is_info_prints_enabled(),
                    print_type,
                    sources_track,
                    github_sources_track,
                    message,
                );
            },
            None => match print_type {
                PrintType::Error => {
                    if CONFIG.is_error_prints_enabled {
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
                    if CONFIG.is_warning_high_prints_enabled {
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
                    if CONFIG.is_warning_low_prints_enabled {
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
                    if CONFIG.is_success_prints_enabled {
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
                    if CONFIG.is_partial_success_prints_enabled {
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
                    if CONFIG.is_time_measurement_prints_enabled {
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
                PrintType::Info => {
                    if CONFIG.is_info_prints_enabled {
                        let rgb_color: ansi_term::Colour =
                            RGB(CONFIG.info_red, CONFIG.info_green, CONFIG.info_blue);
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
            },
        }
    }
}
