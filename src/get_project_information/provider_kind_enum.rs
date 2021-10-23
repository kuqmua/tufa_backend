use std::collections::HashMap;

use crate::get_project_information::get_config::get_lazy_config_information::CONFIG;
use crate::get_project_information::project_constants::ARXIV_NAME_TO_CHECK;
use crate::get_project_information::project_constants::BIORXIV_NAME_TO_CHECK;
use crate::get_project_information::project_constants::GITHUB_NAME_TO_CHECK;
use crate::get_project_information::project_constants::HABR_NAME_TO_CHECK;
use crate::get_project_information::project_constants::MEDRXIV_NAME_TO_CHECK;
use crate::get_project_information::project_constants::REDDIT_NAME_TO_CHECK;
use crate::get_project_information::project_constants::TWITTER_NAME_TO_CHECK;
use procedural_macros_lib::EnumVariantCount;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(
    EnumVariantCount,
    EnumIter,
    Clone,
    Debug,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    PartialEq,
    Eq,
    Hash,
    Copy,
)]
pub enum ProviderKind {
    Arxiv,
    Biorxiv,
    Github,
    Habr,
    Medrxiv,
    Reddit,
    Twitter,
}

impl ProviderKind {
    pub fn get_string_name(provider_kind_enum_type: ProviderKind) -> &'static str {
        match provider_kind_enum_type {
            ProviderKind::Arxiv => ARXIV_NAME_TO_CHECK,
            ProviderKind::Biorxiv => BIORXIV_NAME_TO_CHECK,
            ProviderKind::Github => GITHUB_NAME_TO_CHECK,
            ProviderKind::Habr => HABR_NAME_TO_CHECK,
            ProviderKind::Medrxiv => MEDRXIV_NAME_TO_CHECK,
            ProviderKind::Reddit => REDDIT_NAME_TO_CHECK,
            ProviderKind::Twitter => TWITTER_NAME_TO_CHECK,
        }
    }
    pub fn get_length() -> usize {
        ENUM_LENGTH
    }
    pub fn into_vec() -> Vec<ProviderKind> {
        let mut provider_kind_vec = Vec::with_capacity(ENUM_LENGTH);
        for provider_kind in ProviderKind::iter() {
            provider_kind_vec.push(provider_kind);
        }
        provider_kind_vec
    }
    pub fn into_string_name_and_kind_tuple_vec() -> Vec<(&'static str, ProviderKind)> {
        let mut provider_kind_vec = Vec::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter() {
            provider_kind_vec.push((ProviderKind::get_string_name(provider_kind),   provider_kind));
        }
        provider_kind_vec
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn into_string_name_and_kind_hashmap() -> HashMap<String, ProviderKind> {
        //its String coz legacy
        let mut config_provider_string_to_enum_struct_hasmap: HashMap<String, ProviderKind> =
        HashMap::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter() {
            config_provider_string_to_enum_struct_hasmap.insert(ProviderKind::get_string_name(provider_kind).to_owned(),   provider_kind);
        }
        config_provider_string_to_enum_struct_hasmap
    }
    pub fn get_links_limit_for_provider(provider_kind: ProviderKind) -> i64 {
        match provider_kind {
            ProviderKind::Arxiv => CONFIG.providers_links_limits.links_limit_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.providers_links_limits.links_limit_for_biorxiv,
            ProviderKind::Github => CONFIG.providers_links_limits.links_limit_for_github,
            ProviderKind::Habr => CONFIG.providers_links_limits.links_limit_for_habr,
            ProviderKind::Medrxiv => CONFIG.providers_links_limits.links_limit_for_medrxiv,
            ProviderKind::Reddit => CONFIG.providers_links_limits.links_limit_for_reddit,
            ProviderKind::Twitter => CONFIG.providers_links_limits.links_limit_for_twitter,
        }
    }
    pub fn is_provider_kind_string_exists(potential_provider_kind_string: &str) -> bool {
        for provider_kind in ProviderKind::iter() {
            let provider_kind_string = ProviderKind::get_string_name(provider_kind);
            if provider_kind_string == potential_provider_kind_string {
                return true;
            }
        }
        false
    }
    pub fn get_provider_kind_array_from_string_vec(
        providers_vec_of_strings: Vec<String>,
    ) -> Vec<ProviderKind> {
        let mut provider_kind_vec: Vec<ProviderKind> =
            Vec::with_capacity(providers_vec_of_strings.len());
        for potential_provider_string in providers_vec_of_strings {
            match potential_provider_string.as_ref() {
                ARXIV_NAME_TO_CHECK => provider_kind_vec.push(ProviderKind::Arxiv),
                BIORXIV_NAME_TO_CHECK => provider_kind_vec.push(ProviderKind::Biorxiv),
                GITHUB_NAME_TO_CHECK => provider_kind_vec.push(ProviderKind::Github),
                HABR_NAME_TO_CHECK => provider_kind_vec.push(ProviderKind::Habr),
                MEDRXIV_NAME_TO_CHECK => provider_kind_vec.push(ProviderKind::Medrxiv),
                REDDIT_NAME_TO_CHECK => provider_kind_vec.push(ProviderKind::Reddit),
                TWITTER_NAME_TO_CHECK => provider_kind_vec.push(ProviderKind::Twitter),
                _ => {
                    //todo: cannot use print_colorful_message coz cyclic package dependency with prints_lib
                    panic!("potential_provider_string is not defined")
                }
            }
        }
        provider_kind_vec
    }
}
