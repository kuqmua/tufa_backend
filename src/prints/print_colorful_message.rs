use super::print_wrapper::print_wrapper;
use crate::global_variables::runtime::config::CONFIG;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::providers::provider_kind::provider_kind_enum::ProviderKindFromConfig;
use tufa_common::config_mods::config_struct::ConfigStruct;
use tufa_common::config_mods::print_type::PrintType;
use tufa_common::config_mods::tracing_type::TracingType;
use tufa_common::traits::print_type_methods::PrintTypeMethods;

//ansi_term::Colour
pub fn print_colorful_message(
    pk: Option<&ProviderKind>,
    pt: PrintType,
    sources: Vec<String>,
    github_sources: Vec<String>,
    message: String,
) {
    if CONFIG.is_prints_enabled {
        let mut sources_track = String::from("");
        for source in sources {
            sources_track.push_str(&format!("{source}\n"))
        }
        let mut github_sources_track = String::from("");
        for github_source in github_sources {
            github_sources_track.push_str(&format!("{github_source}\n"))
        }
        match pk {
            Some(pk) => {
                if pk.is_prints_enabled() && CONFIG.is_prints_enabled(&pt) {
                    //pk.is_prints_for_print_type_enabled(&pt)
                    print_wrapper(
                        CONFIG.get_color(&pt),
                        sources_track,
                        github_sources_track,
                        message,
                    );
                }
            }
            None => {
                if CONFIG.is_prints_enabled(&pt) {
                    print_wrapper(
                        CONFIG.get_color(&pt),
                        sources_track,
                        github_sources_track,
                        message,
                    );
                }
            }
        }
    }
}
