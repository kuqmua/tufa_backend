use ansi_term::Colour;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn print_wrapper(
    color: Colour,
    sources_track: String,
    github_sources_track: String,
    message: String,
) {
    match crate::config_mods::lazy_static_config::CONFIG.source_place_type {
        crate::config_mods::source_place_type::SourcePlaceType::Source => {
            eprintln!(
                "{}\n{}",
                color.bold().paint(sources_track),
                color.bold().paint(message)
            );
        }
        crate::config_mods::source_place_type::SourcePlaceType::Github => {
            eprintln!(
                "{}\n{}",
                color.bold().paint(github_sources_track),
                color.bold().paint(message)
            );
        }
        crate::config_mods::source_place_type::SourcePlaceType::None => {
            eprintln!("{}", color.bold().paint(message));
        }
    }
}
