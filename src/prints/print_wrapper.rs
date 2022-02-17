use ansi_term::Colour;

use crate::config_mods::lazy_static_config::CONFIG;

pub fn print_wrapper(
    color: Colour,
    sources_track: String,
    github_sources_track: String,
    message: String,
) {
    if CONFIG.is_show_source_place_enabled && CONFIG.is_show_github_source_place_enabled {
        eprintln!(
            "{}{}\n{}",
            color.bold().paint(sources_track),
            color.bold().paint(github_sources_track),
            color.bold().paint(message)
        );
    } else if CONFIG.is_show_source_place_enabled {
        eprintln!(
            "{}\n{}",
            color.bold().paint(sources_track),
            color.bold().paint(message)
        );
    } else if CONFIG.is_show_github_source_place_enabled {
        eprintln!(
            "{}\n{}",
            color.bold().paint(github_sources_track),
            color.bold().paint(message)
        );
    } else {
        eprintln!("{}", color.bold().paint(message));
    }
}
