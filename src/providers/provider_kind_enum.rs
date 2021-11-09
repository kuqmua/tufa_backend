use std::collections::HashMap;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client,
};

use std::fs;

use crate::{config_mods::config::CONFIG, fetch::rss_clean_logs_directory::CleanLogsDirError};
use crate::constants::project_constants::ARXIV_NAME_TO_CHECK;
use crate::constants::project_constants::BIORXIV_NAME_TO_CHECK;
use crate::constants::project_constants::GITHUB_NAME_TO_CHECK;
use crate::constants::project_constants::HABR_NAME_TO_CHECK;
use crate::constants::project_constants::MEDRXIV_NAME_TO_CHECK;
use crate::constants::project_constants::REDDIT_NAME_TO_CHECK;
use crate::constants::project_constants::TWITTER_NAME_TO_CHECK;

use procedural_macros_lib::EnumVariantCount;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;
use crate::mongo_integration::mongo_possibly_get_documents_as_string_vector::mongo_possibly_get_documents_as_string_vector;

use crate::providers::providers_info::providers_init_json_schema::ProvidersInitJsonSchema;

use crate::fetch::rss_clean_logs_directory::rss_clean_logs_directory;

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
        let mut string_name_vec: Vec<&'static str> = Vec::with_capacity(ProviderKind::get_length());
        for provider_kind in
            ProviderKind::iter().filter(|element| ProviderKind::is_enabled(*element))
        {
            string_name_vec.push(ProviderKind::get_string_name(provider_kind));
        }
        string_name_vec
    }
    pub fn get_enabled_providers_vec() -> Vec<ProviderKind> {
        let mut providers_vec: Vec<ProviderKind> = Vec::with_capacity(ProviderKind::get_length());
        for provider_kind in
            ProviderKind::iter().filter(|element| ProviderKind::is_enabled(*element))
        {
            providers_vec.push(provider_kind);
        }
        providers_vec
    }
    pub fn get_mongo_initialization_string_name_vec() -> Vec<&'static str> {
        let mut vec_of_filtered_provider_names: Vec<&'static str> =
            Vec::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter()
            .filter(|element| ProviderKind::is_mongo_initialization_enabled(*element))
        {
            vec_of_filtered_provider_names.push(ProviderKind::get_string_name(provider_kind))
        }
        vec_of_filtered_provider_names
    }
    pub fn get_mongo_initialization_provider_kind_vec() -> Vec<ProviderKind> {
        let mut vec_of_filtered_provider_names: Vec<ProviderKind> =
            Vec::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter()
            .filter(|element| ProviderKind::is_mongo_initialization_enabled(*element))
        {
            vec_of_filtered_provider_names.push(provider_kind)
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
            config_provider_string_to_enum_struct_hasmap
                .insert(ProviderKind::get_string_name(provider_kind), provider_kind);
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
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub async fn mongo_get_provider_link_parts_as_bson_string(
        provider_kind: ProviderKind,
    ) -> Result<Option<Vec<String>>, mongodb::error::Error> {
        //todo maybe option vec string
        let client_options = ClientOptions::parse(mongo_get_db_url()).await?;
        let client = Client::with_options(client_options)?;
        //declare db name. there is no create db method in mongo
        let db = client.database(&CONFIG.mongo_params.providers_db_name_handle);
        let mut needed_db_collection: Option<String> = None;
        for collection_name in db.list_collection_names(None).await? {
            if collection_name == *ProviderKind::get_mongo_collection_name(provider_kind) {
                needed_db_collection = Some(collection_name);
            }
        }
        if let Some(collection_name) = needed_db_collection {
            let collection = db.collection(&collection_name);
            let documents_number = collection.count_documents(None, None).await?;
            if documents_number > 0 {
                //rewrite as PrintType::Info or something
                print_colorful_message(
                    None,
                    PrintType::Success,
                    file!().to_string(),
                    line!().to_string(),
                    format!("collection.count_documents {}", documents_number),
                );
                let option_aggregation_stage_1_get_docs_in_random_order_with_limit: Option<
                    Document,
                >;
                if CONFIG.params.enable_provider_links_limit {
                    if CONFIG.params.enable_common_providers_links_limit {
                        if CONFIG
                            .params
                            .enable_randomize_order_for_providers_link_parts_for_mongo
                        {
                            option_aggregation_stage_1_get_docs_in_random_order_with_limit = Some(
                                doc! { "$sample" : {"size": CONFIG.params.common_providers_links_limit }},
                            );
                        } else {
                            option_aggregation_stage_1_get_docs_in_random_order_with_limit = Some(
                                doc! { "$limit" :  CONFIG.params.common_providers_links_limit },
                            );
                        }
                    } else {
                        option_aggregation_stage_1_get_docs_in_random_order_with_limit =
                            ProviderKind::get_mongo_doc_randomization_aggregation(provider_kind);
                    }
                } else {
                    option_aggregation_stage_1_get_docs_in_random_order_with_limit = None;
                }
                // let aggregation_stage_1_get_docs_in_random_order_with_limit =
                //     doc! { "$sample" : {"size": 5 }};
                // let aggregation_stage_2_get_docs_with_limit = doc! { "$limit": 5 };
                let option_vec_of_strings = mongo_possibly_get_documents_as_string_vector(
                    collection,
                    &CONFIG
                        .mongo_params
                        .providers_db_collection_document_field_name_handle,
                    option_aggregation_stage_1_get_docs_in_random_order_with_limit,
                )
                .await?;
                match option_vec_of_strings {
                    Some(vec_of_strings) => return Ok(Some(vec_of_strings)),
                    None => return Ok(None),
                }
            }
        }
        Ok(None)
        // match needed_db_collection {
        //     Some(collection_name) => {
        //         let collection = db.collection(&collection_name);
        //         let documents_number = collection.count_documents(None, None).await?;
        //         if documents_number > 0 {
        //             //rewrite as PrintType::Info or something
        //             print_colorful_message(
        //                 None,
        //                 PrintType::Success,
        //                 file!().to_string(),
        //                 line!().to_string(),
        //                 format!("collection.count_documents {}", documents_number),
        //             );
        //             let option_aggregation_stage_1_get_docs_in_random_order_with_limit: Option<Document>;
        //             if CONFIG.params.enable_provider_links_limit {
        //                 if CONFIG.params.enable_common_providers_links_limit {
        //                     if CONFIG.params.enable_randomize_order_for_providers_link_parts_for_mongo {
        //                         option_aggregation_stage_1_get_docs_in_random_order_with_limit = Some(doc! { "$sample" : {"size": CONFIG.params.common_providers_links_limit }});
        //                     }
        //                     else {
        //                         option_aggregation_stage_1_get_docs_in_random_order_with_limit = Some(doc! { "$limit" :  CONFIG.params.common_providers_links_limit });
        //                     }
        //                 } else {
        //                     option_aggregation_stage_1_get_docs_in_random_order_with_limit = ProviderKind::get_mongo_doc_randomization_aggregation(provider_kind);
        //                 }
        //             } else {
        //                 option_aggregation_stage_1_get_docs_in_random_order_with_limit = None;
        //             }
        //             // let aggregation_stage_1_get_docs_in_random_order_with_limit =
        //             //     doc! { "$sample" : {"size": 5 }};
        //             // let aggregation_stage_2_get_docs_with_limit = doc! { "$limit": 5 };
        //             Ok(Some(mongo_possibly_get_documents_as_string_vector(
        //                 collection,
        //                 &CONFIG.mongo_params.providers_db_collection_document_field_name_handle,
        //                 option_aggregation_stage_1_get_docs_in_random_order_with_limit,
        //             )
        //             .await?))
        //         } else {
        //             print_colorful_message(
        //                 None,
        //                 PrintType::WarningLow,
        //                 file!().to_string(),
        //                 line!().to_string(),
        //                 format!("documents_number is {}", documents_number),
        //             );
        //             Ok(None)
        //         }
        //     }
        //     None => {
        //         print_colorful_message(
        //             None,
        //             PrintType::WarningLow,
        //             file!().to_string(),
        //             line!().to_string(),
        //             "needed_db_collection is None".to_string(),
        //         );
        //         Ok(None)
        //     }
    }
    pub fn enable_links_limit_for(provider_kind: ProviderKind) -> bool {
        match provider_kind {
            ProviderKind::Arxiv => {
                CONFIG
                    .enable_providers_links_limits
                    .enable_links_limit_for_arxiv
            }
            ProviderKind::Biorxiv => {
                CONFIG
                    .enable_providers_links_limits
                    .enable_links_limit_for_biorxiv
            }
            ProviderKind::Github => {
                CONFIG
                    .enable_providers_links_limits
                    .enable_links_limit_for_github
            }
            ProviderKind::Habr => {
                CONFIG
                    .enable_providers_links_limits
                    .enable_links_limit_for_habr
            }
            ProviderKind::Medrxiv => {
                CONFIG
                    .enable_providers_links_limits
                    .enable_links_limit_for_medrxiv
            }
            ProviderKind::Reddit => {
                CONFIG
                    .enable_providers_links_limits
                    .enable_links_limit_for_reddit
            }
            ProviderKind::Twitter => {
                CONFIG
                    .enable_providers_links_limits
                    .enable_links_limit_for_twitter
            }
        }
    }
    pub fn enable_randomize_order_mongo_link_parts_for(provider_kind: ProviderKind) -> bool {
        match provider_kind {
            ProviderKind::Arxiv => {
                CONFIG
                    .enable_randomize_order_for_providers_link_parts_for_mongo
                    .enable_randomize_order_for_arxiv_link_parts_for_mongo
            }
            ProviderKind::Biorxiv => {
                CONFIG
                    .enable_randomize_order_for_providers_link_parts_for_mongo
                    .enable_randomize_order_for_biorxiv_link_parts_for_mongo
            }
            ProviderKind::Github => {
                CONFIG
                    .enable_randomize_order_for_providers_link_parts_for_mongo
                    .enable_randomize_order_for_github_link_parts_for_mongo
            }
            ProviderKind::Habr => {
                CONFIG
                    .enable_randomize_order_for_providers_link_parts_for_mongo
                    .enable_randomize_order_for_habr_link_parts_for_mongo
            }
            ProviderKind::Medrxiv => {
                CONFIG
                    .enable_randomize_order_for_providers_link_parts_for_mongo
                    .enable_randomize_order_for_medrxiv_link_parts_for_mongo
            }
            ProviderKind::Reddit => {
                CONFIG
                    .enable_randomize_order_for_providers_link_parts_for_mongo
                    .enable_randomize_order_for_reddit_link_parts_for_mongo
            }
            ProviderKind::Twitter => {
                CONFIG
                    .enable_randomize_order_for_providers_link_parts_for_mongo
                    .enable_randomize_order_for_twitter_link_parts_for_mongo
            }
        }
    }
    pub fn get_mongo_doc_randomization_aggregation(
        provider_kind: ProviderKind,
    ) -> Option<Document> {
        if ProviderKind::enable_links_limit_for(provider_kind) {
            if ProviderKind::enable_randomize_order_mongo_link_parts_for(provider_kind) {
                Some(
                    doc! { "$sample" : {"size": ProviderKind::get_links_limit_for_provider(provider_kind) }},
                )
            } else {
                Some(doc! { "$limit" : ProviderKind::get_links_limit_for_provider(provider_kind) })
            }
        } else {
            None
        }
    }
    pub fn get_check_link(provider_kind: ProviderKind) -> &'static str {
        match provider_kind {
            ProviderKind::Arxiv => &CONFIG.providers_check_links.arxiv_link,
            ProviderKind::Biorxiv => &CONFIG.providers_check_links.biorxiv_link,
            ProviderKind::Github => &CONFIG.providers_check_links.github_link,
            ProviderKind::Medrxiv => &CONFIG.providers_check_links.medrxiv_link,
            ProviderKind::Twitter => &CONFIG.providers_check_links.twitter_link,
            ProviderKind::Reddit => &CONFIG.providers_check_links.reddit_link,
            ProviderKind::Habr => &CONFIG.providers_check_links.habr_link,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn get_providers_json_local_data() -> HashMap<ProviderKind, Vec<String>> {
        let mut vec_of_link_parts_hashmap: HashMap<ProviderKind, Vec<String>> = HashMap::new();
        //todo: do it async in parallel
        for provider_kind in ProviderKind::get_enabled_providers_vec() {
            let result_of_reading_to_string = fs::read_to_string(&format!(
                "{}{}{}{}",
                CONFIG.mongo_params.path_to_provider_link_parts_folder,
                ProviderKind::get_string_name(provider_kind),
                CONFIG
                    .mongo_params
                    .providers_db_collection_handle_second_part,
                CONFIG.mongo_params.log_file_extension
            ));
            match result_of_reading_to_string {
                Ok(file_content) => {
                    let file_content_from_str_result: Result<
                        ProvidersInitJsonSchema,
                        serde_json::Error,
                    > = serde_json::from_str(&file_content);
                    match file_content_from_str_result {
                        Ok(file_content_as_struct) => {
                            let mut vec_of_link_parts: Vec<String> =
                                Vec::with_capacity(file_content_as_struct.data.len());
                            for link_part in file_content_as_struct.data {
                                vec_of_link_parts.push(link_part)
                            }
                            vec_of_link_parts_hashmap.insert(provider_kind, vec_of_link_parts);
                        }
                        Err(e) => println!("file_content_from_str_result error {:#?}", e),
                    }
                }
                Err(e) => {
                    println!(
                        "cannot read_to_string from file {}{}{}{}, reason: {}",
                        CONFIG.mongo_params.path_to_provider_link_parts_folder,
                        ProviderKind::get_string_name(provider_kind),
                        CONFIG
                            .mongo_params
                            .providers_db_collection_handle_second_part,
                        CONFIG.mongo_params.log_file_extension,
                        e
                    )
                }
            }
        }
        vec_of_link_parts_hashmap
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn is_cleaning_warning_logs_directory_enable(provider_kind: ProviderKind) -> bool {
        match provider_kind {
            ProviderKind::Arxiv => CONFIG
            .enable_providers_cleaning_warning_logs_directory
            .enable_cleaning_warning_logs_directory_for_arxiv,
            ProviderKind::Biorxiv => CONFIG
            .enable_providers_cleaning_warning_logs_directory
            .enable_cleaning_warning_logs_directory_for_biorxiv,
            ProviderKind::Github => CONFIG
            .enable_providers_cleaning_warning_logs_directory
            .enable_cleaning_warning_logs_directory_for_habr,
            ProviderKind::Habr => CONFIG
            .enable_providers_cleaning_warning_logs_directory
            .enable_cleaning_warning_logs_directory_for_habr,
            ProviderKind::Medrxiv => CONFIG
            .enable_providers_cleaning_warning_logs_directory
            .enable_cleaning_warning_logs_directory_for_medrxiv,
            ProviderKind::Reddit => CONFIG
            .enable_providers_cleaning_warning_logs_directory
            .enable_cleaning_warning_logs_directory_for_reddit,
            ProviderKind::Twitter => CONFIG
            .enable_providers_cleaning_warning_logs_directory
            .enable_cleaning_warning_logs_directory_for_twitter
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn clean_providers_logs_directory() -> HashMap::<ProviderKind, CleanLogsDirError> {
        let mut result_hashmap: HashMap::<ProviderKind, CleanLogsDirError> = HashMap::with_capacity(ProviderKind::get_length());
        for provider_kind in
            ProviderKind::iter().filter(|element| ProviderKind::is_cleaning_warning_logs_directory_enable(*element))
        {
            match rss_clean_logs_directory(provider_kind) {
                Ok(_) => {},
                Err(e) => {
                    result_hashmap.insert(provider_kind, e);
                },
            }
        }
        result_hashmap
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn get_path_to_logs_directory(provider_kind: ProviderKind) -> String {
        format!(
            "logs/{}/{:?}",
            &CONFIG.params.warning_logs_directory_name, provider_kind
        )
    }
}
