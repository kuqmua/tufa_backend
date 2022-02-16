use crate::config_mods::lazy_static_config::CONFIG;
use crate::providers::provider_kind_enum::ProviderKind;
use crate::traits::print_type_trait::PrintTypeTrait;
use crate::traits::provider_kind_from_config_trait::ProviderKindFromConfigTrait;

use crate::prints::print_type_enum::PrintType;

use super::print_wrapper::print_wrapper;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn print_colorful_message(
    pk: Option<&ProviderKind>,
    pt: PrintType,
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
                if pk.is_prints_enabled() {
                    if ProviderKind::is_prints_for_print_type_enabled(pk, &pt) {
                        print_wrapper(pt.get_color(), sources_track, github_sources_track, message);
                    }
                }
            },
            None => {
                if pt.is_prints_enabled() {
                    print_wrapper(pt.get_color(), sources_track, github_sources_track, message);
                }
            },
        }
    }
}
