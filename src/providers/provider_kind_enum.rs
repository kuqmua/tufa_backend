use std::collections::HashMap;

use crate::config_mods::config::CONFIG;
use crate::constants::project_constants::ARXIV_NAME_TO_CHECK;
use crate::constants::project_constants::BIORXIV_NAME_TO_CHECK;
use crate::constants::project_constants::GITHUB_NAME_TO_CHECK;
use crate::constants::project_constants::HABR_NAME_TO_CHECK;
use crate::constants::project_constants::MEDRXIV_NAME_TO_CHECK;
use crate::constants::project_constants::REDDIT_NAME_TO_CHECK;
use crate::constants::project_constants::TWITTER_NAME_TO_CHECK;
use procedural_macros_lib::EnumVariantCount;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// use crate::providers::providers_info::get_providers_link_parts::get_providers_link_parts_as_hashmap;

// use crate::helpers::resource::Resource;

// use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;

// use crate::prints::print_colorful_message::print_colorful_message;
// use crate::prints::print_type_enum::PrintType;

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
    pub fn get_string_name(provider_kind: ProviderKind) -> &'static str {
        match provider_kind {
            ProviderKind::Arxiv => ARXIV_NAME_TO_CHECK,
            ProviderKind::Biorxiv => BIORXIV_NAME_TO_CHECK,
            ProviderKind::Github => GITHUB_NAME_TO_CHECK,
            ProviderKind::Habr => HABR_NAME_TO_CHECK,
            ProviderKind::Medrxiv => MEDRXIV_NAME_TO_CHECK,
            ProviderKind::Reddit => REDDIT_NAME_TO_CHECK,
            ProviderKind::Twitter => TWITTER_NAME_TO_CHECK,
        }
    }
    pub fn get_mongo_collection_name(provider_kind: ProviderKind) -> String {
        let name = ProviderKind::get_string_name(provider_kind);
        format!(
            "{}{}",
            name,
            CONFIG
                .mongo_params
                .providers_db_collection_handle_second_part
        )
    }
    pub fn is_enabled(provider_kind: ProviderKind) -> bool {
        match provider_kind {
            ProviderKind::Arxiv => CONFIG.enable_providers.enable_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_providers.enable_biorxiv,
            ProviderKind::Github => CONFIG.enable_providers.enable_github,
            ProviderKind::Habr => CONFIG.enable_providers.enable_habr,
            ProviderKind::Medrxiv => CONFIG.enable_providers.enable_medrxiv,
            ProviderKind::Reddit => CONFIG.enable_providers.enable_reddit,
            ProviderKind::Twitter => CONFIG.enable_providers.enable_twitter,
        }
    }
    pub fn is_mongo_initialization_enabled(provider_kind: ProviderKind) -> bool {
        match provider_kind {
            ProviderKind::Arxiv => {
                CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_arxiv_link_parts
            }
            ProviderKind::Biorxiv => {
                CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_biorxiv_link_parts
            }
            ProviderKind::Github => {
                CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_github_link_parts
            }
            ProviderKind::Habr => {
                CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_habr_link_parts
            }
            ProviderKind::Medrxiv => {
                CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_medrxiv_link_parts
            }
            ProviderKind::Reddit => {
                CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_reddit_link_parts
            }
            ProviderKind::Twitter => {
                CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_twitter_link_parts
            }
        }
    }
    pub fn stringify(provider_kind: ProviderKind) -> &'static str {
        match provider_kind {
            ProviderKind::Arxiv => stringify!(ProviderKind::Arxiv),
            ProviderKind::Biorxiv => stringify!(ProviderKind::Arxiv),
            ProviderKind::Github => stringify!(ProviderKind::Arxiv),
            ProviderKind::Habr => stringify!(ProviderKind::Arxiv),
            ProviderKind::Medrxiv => stringify!(ProviderKind::Arxiv),
            ProviderKind::Reddit => stringify!(ProviderKind::Arxiv),
            ProviderKind::Twitter => stringify!(ProviderKind::Arxiv),
        }
    }
    pub fn get_length() -> usize {
        ENUM_LENGTH
    }
    pub fn get_enabled_string_name_vec() -> Vec<&'static str> {
        let mut string_name_vec: Vec<&'static str> =
        Vec::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter().filter(|element| ProviderKind::is_enabled(*element)) {
            string_name_vec.push(ProviderKind::get_string_name(provider_kind));
        }
        string_name_vec
    }
    pub fn get_mongo_initialization_vec() -> Vec<&'static str> {
        let mut vec_of_filtered_provider_names: Vec<&'static str> =
            Vec::with_capacity(ProviderKind::get_length());
        for provider_kind in
            ProviderKind::iter().filter(|element| ProviderKind::is_mongo_initialization_enabled(*element))
        {
            vec_of_filtered_provider_names.push(ProviderKind::get_string_name(provider_kind))
        }
        vec_of_filtered_provider_names
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
            provider_kind_vec.push((ProviderKind::get_string_name(provider_kind), provider_kind));
        }
        provider_kind_vec
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn into_string_name_and_kind_hashmap() -> HashMap<&'static str, ProviderKind> {
        //its String coz legacy
        let mut config_provider_string_to_enum_struct_hasmap: HashMap<&'static str, ProviderKind> =
            HashMap::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter() {
            config_provider_string_to_enum_struct_hasmap.insert(
                ProviderKind::get_string_name(provider_kind),
                provider_kind,
            );
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
    // ////////
    // pub async fn get_link_parts(resource: &Resource, provider_kind: ProviderKind) -> Option<Vec<String>> {
    //     let providers_link_parts = get_providers_link_parts_as_hashmap(resource).await;
    //     if !providers_link_parts.is_empty() {
    //         Some(providers_link_parts)
    //     } else {
    //         print_colorful_message(
    //             None,
    //             PrintType::WarningLow,
    //             file!().to_string(),
    //             line!().to_string(),
    //             "providers_link_parts .is_empty".to_string(),
    //         );
    //         let providers_link_parts_local = get_providers_link_parts_as_hashmap(&Resource::Local {
    //             path_to_provider_link_parts_folder: CONFIG
    //                 .mongo_params
    //                 .path_to_provider_link_parts_folder
    //                 .to_string(),
    //             vec_of_provider_names: CONFIG.params.vec_of_provider_names.clone(),
    //             second_part_of_file_name: CONFIG
    //                 .mongo_params
    //                 .providers_db_collection_handle_second_part //why that in mongo_params?
    //                 .to_string(),
    //             file_extension: CONFIG.mongo_params.log_file_extension.to_string(),
    //         })
    //         .await;
    //         if !providers_link_parts_local.is_empty() {
    //             Some(providers_link_parts_local)
    //         } else {
    //             print_colorful_message(
    //                 None,
    //                 PrintType::Error,
    //                 file!().to_string(),
    //                 line!().to_string(),
    //                 "providers_link_parts_local.is_empty too".to_string(),
    //             );
    //             None
    //         }
    //     }
    // }
    // pub async fn get_providers_link_parts_wrapper() -> Option<HashMap<String, Vec<String>>> {
    //     let mongo_url = mongo_get_db_url();
    //     let providers_string_into_enum_hashmap: HashMap<String, ProviderKind> =
    //         ProviderKind::into_string_name_and_kind_hashmap();
    //     let providers_link_parts = get_providers_link_parts_as_hashmap(&Resource::Mongodb {
    //         mongo_url,
    //         db_name_handle: CONFIG.mongo_params.providers_db_name_handle.to_string(),
    //         db_collection_handle_second_part: CONFIG
    //             .mongo_params
    //             .providers_db_collection_handle_second_part
    //             .to_string(),
    //         db_collection_document_field_name_handle: CONFIG
    //             .mongo_params
    //             .providers_db_collection_document_field_name_handle
    //             .to_string(),
    //         providers_string_into_enum_hashmap,
    //     })
    //     .await;
    //     if !providers_link_parts.is_empty() {
    //         Some(providers_link_parts)
    //     } else {
    //         print_colorful_message(
    //             None,
    //             PrintType::WarningLow,
    //             file!().to_string(),
    //             line!().to_string(),
    //             "providers_link_parts .is_empty".to_string(),
    //         );
    //         let providers_link_parts_local = get_providers_link_parts_as_hashmap(&Resource::Local {
    //             path_to_provider_link_parts_folder: CONFIG
    //                 .mongo_params
    //                 .path_to_provider_link_parts_folder
    //                 .to_string(),
    //             vec_of_provider_names: CONFIG.params.vec_of_provider_names.clone(),
    //             second_part_of_file_name: CONFIG
    //                 .mongo_params
    //                 .providers_db_collection_handle_second_part //why that in mongo_params?
    //                 .to_string(),
    //             file_extension: CONFIG.mongo_params.log_file_extension.to_string(),
    //         })
    //         .await;
    //         if !providers_link_parts_local.is_empty() {
    //             Some(providers_link_parts_local)
    //         } else {
    //             print_colorful_message(
    //                 None,
    //                 PrintType::Error,
    //                 file!().to_string(),
    //                 line!().to_string(),
    //                 "providers_link_parts_local.is_empty too".to_string(),
    //             );
    //             None
    //         }
    //     }
    // }
}
