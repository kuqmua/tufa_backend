use std::{collections::HashMap, path::Path};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use mongodb::bson::doc;

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
