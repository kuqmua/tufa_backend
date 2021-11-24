use std::{collections::HashMap, path::Path};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use mongodb::{
    bson::{doc, Document}
};

use std::fs;

use crate::config_mods::config::CONFIG;

use crate::constants::project_constants::ARXIV_PROVIDER_ITEM_HANDLE;
use crate::constants::project_constants::BIORXIV_PROVIDER_ITEM_HANDLE;
use crate::constants::project_constants::GITHUB_PROVIDER_ITEM_HANDLE;
use crate::constants::project_constants::HABR_PROVIDER_ITEM_HANDLE;
use crate::constants::project_constants::MEDRXIV_PROVIDER_ITEM_HANDLE;
use crate::constants::project_constants::TWITTER_PROVIDER_ITEM_HANDLE;

use procedural_macros_lib::EnumVariantCount;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::providers::providers_info::providers_init_json_schema::ProvidersInitJsonSchema;

use crate::providers::providers_info::links::generate_arxiv_links::generate_arxiv_links;
use crate::providers::providers_info::links::generate_biorxiv_links::generate_biorxiv_links;
use crate::providers::providers_info::links::generate_github_links::generate_github_links;
use crate::providers::providers_info::links::generate_habr_links::generate_habr_links;
use crate::providers::providers_info::links::generate_medrxiv_links::generate_medrxiv_links;
use crate::providers::providers_info::links::generate_reddit_links::generate_reddit_links;
use crate::providers::providers_info::links::generate_twitter_links::generate_twitter_links;

#[derive(Debug)]
pub struct RemoveDirError {
    pub error: std::io::Error,
}

#[derive(Debug)]
pub enum CleanLogsDirError {
    PathIsNotDir { path: String },
    CannotRemoveDir { error: RemoveDirError },
}
impl From<String> for CleanLogsDirError {
    fn from(e: String) -> Self {
        CleanLogsDirError::PathIsNotDir { path: e }
    }
}
impl From<std::io::Error> for CleanLogsDirError {
    fn from(e: std::io::Error) -> Self {
        CleanLogsDirError::CannotRemoveDir {
            error: RemoveDirError { error: e },
        }
    }
}

#[derive(Debug)]
pub enum GetProvidersJsonLocalDataProcessedError {
    SerdeJsonErrors(Vec<serde_json::Error>),
    StdIoError(std::io::Error),
}

#[derive(Debug)]
pub enum MongoGetProvidersLinkPartsProcessedResult {
    MongoDocuments(HashMap<ProviderKind, mongodb::error::Error>),
    MongoConnection(mongodb::error::Error),
}

#[derive(EnumVariantCount, EnumIter, Clone, Debug, serde_derive::Serialize, serde_derive::Deserialize, PartialEq, Eq, Hash, Copy)]
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
    pub fn get_length() -> usize {
        ENUM_LENGTH
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub async fn mongo_get_providers_link_parts_processed() -> (HashMap<ProviderKind, Vec<String>>, MongoGetProvidersLinkPartsProcessedResult) {//HashMap<ProviderKind, Vec<String>>
        match ProviderKind::mongo_get_providers_link_parts_unprocessed().await {
            Ok(unprocessed_hashmap) => {
                let mut first_return_handle: HashMap<ProviderKind, Vec<String>> = HashMap::with_capacity(unprocessed_hashmap.len());
                let mut second_return_handle: HashMap<ProviderKind, mongodb::error::Error> = HashMap::with_capacity(unprocessed_hashmap.len());
                for (provider_kind, result_vec) in unprocessed_hashmap {
                    match result_vec {
                        Ok(vec) => {
                            first_return_handle.insert(provider_kind, vec);
                        }
                        Err(e) => {
                            print_colorful_message(
                                    Some(&provider_kind),
                                    PrintType::Error,
                                    file!().to_string(),
                                    line!().to_string(),
                                    format!("(todo!)ProviderKind::mongo_get_providers_link_parts_processed ({:#?}), error: {:#?}", provider_kind, e),
                                );
                                second_return_handle
                                .insert(provider_kind, e);
                        }
                    }
                }
                (first_return_handle, MongoGetProvidersLinkPartsProcessedResult::MongoDocuments(second_return_handle))
            }
            Err(e) => {
                print_colorful_message(
                    None,
                    PrintType::Error,
                    file!().to_string(),
                    line!().to_string(),
                    format!(
                        "(todo!)ProviderKind::mongo_get_providers_link_parts_processed error: {:#?}",
                        e
                    ),
                );
                (HashMap::new(), MongoGetProvidersLinkPartsProcessedResult::MongoConnection(e))
            }
        }
    }
    //////////
    pub fn generate_hashmap_with_empty_string_vecs_for_enabled_providers(
    ) -> HashMap<ProviderKind, Vec<String>> {
        let mut hashmap_with_empty_vecs = HashMap::<ProviderKind, Vec<String>>::with_capacity(
            ProviderKind::get_enabled_providers_vec().len(),
        );
        for provider_kind in ProviderKind::get_enabled_providers_vec().iter() {
            hashmap_with_empty_vecs.insert(*provider_kind, Vec::<String>::new());
        }
        hashmap_with_empty_vecs
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
    pub fn get_init_local_data_file_path(provider_kind: ProviderKind) -> String {
        format!(
            "{}{}{}{}",
            CONFIG.mongo_params.path_to_provider_link_parts_folder,
            ProviderKind::get_string_name(provider_kind),
            CONFIG
                .mongo_params
                .providers_db_collection_handle_second_part,
            CONFIG.mongo_params.log_file_extension
        )
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn get_providers_json_local_data_unprocessed(
    ) -> HashMap<ProviderKind, Result<Result<Vec<String>, serde_json::Error>, std::io::Error>> {
        let mut vec_of_link_parts_hashmap: HashMap<
            ProviderKind,
            Result<Result<Vec<String>, serde_json::Error>, std::io::Error>,
        > = HashMap::with_capacity(ProviderKind::get_enabled_providers_vec().len());
        //todo: do it async in parallel
        for provider_kind in ProviderKind::get_enabled_providers_vec() {
            let result_of_reading_to_string =
                fs::read_to_string(&ProviderKind::get_init_local_data_file_path(provider_kind));
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
                            vec_of_link_parts_hashmap
                                .insert(provider_kind, Ok(Ok(vec_of_link_parts)));
                        }
                        Err(e) => {
                            vec_of_link_parts_hashmap.insert(provider_kind, Ok(Err(e)));
                        }
                    }
                }
                Err(e) => {
                    vec_of_link_parts_hashmap.insert(provider_kind, Err(e));
                }
            }
        }
        vec_of_link_parts_hashmap
    }
    ////////////////////
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn get_providers_json_local_data_processed() -> (
        HashMap<ProviderKind, Vec<String>>,
        HashMap<ProviderKind, GetProvidersJsonLocalDataProcessedError>,
    ) {
        let unprocessed_hashmap = ProviderKind::get_providers_json_local_data_unprocessed();
        let mut first_return_handle: HashMap<ProviderKind, Vec<String>> =
            HashMap::with_capacity(unprocessed_hashmap.len());
        let mut second_return_handle: HashMap<
            ProviderKind,
            GetProvidersJsonLocalDataProcessedError,
        > = HashMap::with_capacity(unprocessed_hashmap.len());
        for (provider_kind, result) in unprocessed_hashmap {
            match result {
                Ok(second_result) => {
                    let mut serde_json_error_vec = Vec::<serde_json::Error>::new();
                    match second_result {
                        Ok(vec) => {
                            first_return_handle.insert(provider_kind, vec);
                        }
                        Err(e) => {
                            print_colorful_message(
                                    Some(&provider_kind),
                                    PrintType::Error,
                                    file!().to_string(),
                                    line!().to_string(),
                                    format!("(todo!)ProviderKind::get_providers_json_local_data_unprocessed ({:#?}), error: {:#?}", provider_kind, e),
                                );
                            serde_json_error_vec.push(e);
                        }
                    }
                    if !serde_json_error_vec.is_empty() {
                        second_return_handle.insert(
                            provider_kind,
                            GetProvidersJsonLocalDataProcessedError::SerdeJsonErrors(
                                serde_json_error_vec,
                            ),
                        );
                    }
                }
                Err(e) => {
                    print_colorful_message(
                                    Some(&provider_kind),
                                    PrintType::Error,
                                    file!().to_string(),
                                    line!().to_string(),
                                    format!("(todo!)ProviderKind::get_providers_json_local_data_unprocessed ({:#?}), error: {:#?}", provider_kind, e),
                                );
                    second_return_handle.insert(
                        provider_kind,
                        GetProvidersJsonLocalDataProcessedError::StdIoError(e),
                    );
                }
            }
        }
        (first_return_handle, second_return_handle)
    }
    /////////////////
    //todo add errors warning low warning high info and others
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn is_cleaning_warning_logs_directory_enable(provider_kind: ProviderKind) -> bool {
        match provider_kind {
            ProviderKind::Arxiv => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_directory
                    .enable_cleaning_warning_logs_directory_for_arxiv
            }
            ProviderKind::Biorxiv => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_directory
                    .enable_cleaning_warning_logs_directory_for_biorxiv
            }
            ProviderKind::Github => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_directory
                    .enable_cleaning_warning_logs_directory_for_habr
            }
            ProviderKind::Habr => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_directory
                    .enable_cleaning_warning_logs_directory_for_habr
            }
            ProviderKind::Medrxiv => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_directory
                    .enable_cleaning_warning_logs_directory_for_medrxiv
            }
            ProviderKind::Reddit => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_directory
                    .enable_cleaning_warning_logs_directory_for_reddit
            }
            ProviderKind::Twitter => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_directory
                    .enable_cleaning_warning_logs_directory_for_twitter
            }
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn remove_providers_logs_directories(
    ) -> Result<(), HashMap<ProviderKind, CleanLogsDirError>> {
        let mut result_hashmap: HashMap<ProviderKind, CleanLogsDirError> =
            HashMap::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter()
            .filter(|element| ProviderKind::is_cleaning_warning_logs_directory_enable(*element))
        {
            if let Err(e) = ProviderKind::remove_logs_directory(provider_kind) {
                result_hashmap.insert(provider_kind, e);
            }
        }
        if result_hashmap.is_empty() {
            Ok(())
        } else {
            Err(result_hashmap)
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn remove_existing_providers_logs_directories(
    ) -> Result<(), HashMap<ProviderKind, RemoveDirError>> {
        if let Err(error_hashmap) = ProviderKind::remove_providers_logs_directories() {
            let mut return_hashmap = HashMap::with_capacity(error_hashmap.len());
            for (provider_kind, error) in error_hashmap {
                if let CleanLogsDirError::CannotRemoveDir { error: e } = error {
                    return_hashmap.insert(provider_kind, e);
                }
            }
            if return_hashmap.is_empty() {
                return Ok(());
            }
            return Err(return_hashmap);
        }
        Ok(())
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn get_path_to_logs_directory(provider_kind: ProviderKind) -> String {
        format!(
            "logs/{}/{:?}",
            &CONFIG.params.warning_logs_directory_name, provider_kind
        )
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn get_path_to_provider_log_file(provider_kind: ProviderKind) -> String {
        format!(
            "logs/{}/{:?}/{}",
            &CONFIG.params.warning_logs_directory_name,
            provider_kind,
            &CONFIG
                .params
                .unhandled_success_handled_success_are_there_items_initialized_posts_dir
        )
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn get_provider_links(
        provider_kind: ProviderKind,
        names_vector: Vec<String>,
    ) -> Vec<String> {
        match provider_kind {
            ProviderKind::Arxiv => generate_arxiv_links(names_vector),
            ProviderKind::Biorxiv => generate_biorxiv_links(names_vector),
            ProviderKind::Github => generate_github_links(names_vector),
            ProviderKind::Habr => generate_habr_links(names_vector),
            ProviderKind::Medrxiv => generate_medrxiv_links(names_vector),
            ProviderKind::Reddit => generate_reddit_links(names_vector),
            ProviderKind::Twitter => generate_twitter_links(names_vector),
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn remove_logs_directory(provider_kind: ProviderKind) -> Result<(), CleanLogsDirError> {
        let path = ProviderKind::get_path_to_logs_directory(provider_kind);
        if !Path::new(&path).is_dir() {
            return Err(CleanLogsDirError::PathIsNotDir { path });
        }
        fs::remove_dir_all(&path)?;
        Ok(())
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn get_item_handle(provider_kind: ProviderKind) -> Option<&'static str> {
        match provider_kind {
            ProviderKind::Arxiv => Some(ARXIV_PROVIDER_ITEM_HANDLE),
            ProviderKind::Biorxiv => Some(BIORXIV_PROVIDER_ITEM_HANDLE),
            ProviderKind::Github => Some(GITHUB_PROVIDER_ITEM_HANDLE),
            ProviderKind::Habr => Some(HABR_PROVIDER_ITEM_HANDLE),
            ProviderKind::Medrxiv => Some(MEDRXIV_PROVIDER_ITEM_HANDLE),
            ProviderKind::Reddit => None,
            ProviderKind::Twitter => Some(TWITTER_PROVIDER_ITEM_HANDLE),
        }
    }
}
