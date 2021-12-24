use std::collections::HashMap;
use std::fs;
use std::path::Path;

use mongodb::bson::{doc, Document};
use strum::IntoEnumIterator;

use crate::traits::enum_extention::EnumExtenstion;

use crate::config_mods::lazy_static_config::CONFIG;

use crate::traits::provider_kind_trait::ProviderKindTrait;

use crate::providers::provider_kind_enum::CleanLogsDirError;
use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::provider_kind_enum::RemoveDirError;
use crate::providers::providers_info::links::generate_arxiv_links::generate_arxiv_links;
use crate::providers::providers_info::links::generate_biorxiv_links::generate_biorxiv_links;
use crate::providers::providers_info::links::generate_github_links::generate_github_links;
use crate::providers::providers_info::links::generate_habr_links::generate_habr_links;
use crate::providers::providers_info::links::generate_medrxiv_links::generate_medrxiv_links;
use crate::providers::providers_info::links::generate_reddit_links::generate_reddit_links;
use crate::providers::providers_info::links::generate_twitter_links::generate_twitter_links;

impl ProviderKindTrait for ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_mongo_initialization_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.mongo_enable_initialization_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.mongo_enable_initialization_for_biorxiv,
            ProviderKind::Github => CONFIG.mongo_enable_initialization_for_github,
            ProviderKind::Medrxiv => CONFIG.mongo_enable_initialization_for_medrxiv,
            ProviderKind::Twitter => CONFIG.mongo_enable_initialization_for_twitter,
            ProviderKind::Reddit => CONFIG.mongo_enable_initialization_for_reddit,
            ProviderKind::Habr => CONFIG.mongo_enable_initialization_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_mongo_write_error_logs_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.mongo_enable_write_error_logs_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.mongo_enable_write_error_logs_for_biorxiv,
            ProviderKind::Github => CONFIG.mongo_enable_write_error_logs_for_github,
            ProviderKind::Medrxiv => CONFIG.mongo_enable_write_error_logs_for_medrxiv,
            ProviderKind::Twitter => CONFIG.mongo_enable_write_error_logs_for_twitter,
            ProviderKind::Reddit => CONFIG.mongo_enable_write_error_logs_for_reddit,
            ProviderKind::Habr => CONFIG.mongo_enable_write_error_logs_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_mongo_cleaning_warning_logs_db_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.mongo_enable_cleaning_warning_logs_db_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.mongo_enable_cleaning_warning_logs_db_for_biorxiv,
            ProviderKind::Github => CONFIG.mongo_enable_cleaning_warning_logs_db_for_github,
            ProviderKind::Medrxiv => CONFIG.mongo_enable_cleaning_warning_logs_db_for_medrxiv,
            ProviderKind::Twitter => CONFIG.mongo_enable_cleaning_warning_logs_db_for_twitter,
            ProviderKind::Reddit => CONFIG.mongo_enable_cleaning_warning_logs_db_for_reddit,
            ProviderKind::Habr => CONFIG.mongo_enable_cleaning_warning_logs_db_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_mongo_cleaning_warning_logs_db_collections_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => {
                CONFIG.mongo_enable_cleaning_warning_logs_db_collections_for_arxiv
            }
            ProviderKind::Biorxiv => {
                CONFIG.mongo_enable_cleaning_warning_logs_db_collections_for_biorxiv
            }
            ProviderKind::Github => {
                CONFIG.mongo_enable_cleaning_warning_logs_db_collections_for_github
            }
            ProviderKind::Medrxiv => {
                CONFIG.mongo_enable_cleaning_warning_logs_db_collections_for_medrxiv
            }
            ProviderKind::Twitter => {
                CONFIG.mongo_enable_cleaning_warning_logs_db_collections_for_twitter
            }
            ProviderKind::Reddit => {
                CONFIG.mongo_enable_cleaning_warning_logs_db_collections_for_reddit
            }
            ProviderKind::Habr => CONFIG.mongo_enable_cleaning_warning_logs_db_collections_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_mongo_link_parts_randomize_order_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.mongo_enable_link_parts_randomize_order_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.mongo_enable_link_parts_randomize_order_for_biorxiv,
            ProviderKind::Github => CONFIG.mongo_enable_link_parts_randomize_order_for_github,
            ProviderKind::Medrxiv => CONFIG.mongo_enable_link_parts_randomize_order_for_medrxiv,
            ProviderKind::Twitter => CONFIG.mongo_enable_link_parts_randomize_order_for_twitter,
            ProviderKind::Reddit => CONFIG.mongo_enable_link_parts_randomize_order_for_reddit,
            ProviderKind::Habr => CONFIG.mongo_enable_link_parts_randomize_order_for_habr,
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_postgres_initialization_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.postgres_enable_initialization_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.postgres_enable_initialization_for_biorxiv,
            ProviderKind::Github => CONFIG.postgres_enable_initialization_for_github,
            ProviderKind::Medrxiv => CONFIG.postgres_enable_initialization_for_medrxiv,
            ProviderKind::Twitter => CONFIG.postgres_enable_initialization_for_twitter,
            ProviderKind::Reddit => CONFIG.postgres_enable_initialization_for_reddit,
            ProviderKind::Habr => CONFIG.postgres_enable_initialization_for_habr,
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_write_error_logs_in_local_folder_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_write_error_logs_in_local_folder_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_write_error_logs_in_local_folder_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_write_error_logs_in_local_folder_for_github,
            ProviderKind::Medrxiv => CONFIG.enable_write_error_logs_in_local_folder_for_medrxiv,
            ProviderKind::Twitter => CONFIG.enable_write_error_logs_in_local_folder_for_twitter,
            ProviderKind::Reddit => CONFIG.enable_write_error_logs_in_local_folder_for_reddit,
            ProviderKind::Habr => CONFIG.enable_write_error_logs_in_local_folder_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_cleaning_warning_logs_directory_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_cleaning_warning_logs_directory_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_cleaning_warning_logs_directory_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_cleaning_warning_logs_directory_for_github,
            ProviderKind::Medrxiv => CONFIG.enable_cleaning_warning_logs_directory_for_medrxiv,
            ProviderKind::Twitter => CONFIG.enable_cleaning_warning_logs_directory_for_twitter,
            ProviderKind::Reddit => CONFIG.enable_cleaning_warning_logs_directory_for_reddit,
            ProviderKind::Habr => CONFIG.enable_cleaning_warning_logs_directory_for_habr,
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_check_link(&self) -> &'static str {
        match self {
            ProviderKind::Arxiv => &CONFIG.arxiv_check_link,
            ProviderKind::Biorxiv => &CONFIG.biorxiv_check_link,
            ProviderKind::Github => &CONFIG.github_check_link,
            ProviderKind::Medrxiv => &CONFIG.medrxiv_check_link,
            ProviderKind::Twitter => &CONFIG.twitter_check_link,
            ProviderKind::Reddit => &CONFIG.reddit_check_link,
            ProviderKind::Habr => &CONFIG.habr_check_link,
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_biorxiv,
            ProviderKind::Github => CONFIG.enable_github,
            ProviderKind::Medrxiv => CONFIG.enable_medrxiv,
            ProviderKind::Twitter => CONFIG.enable_twitter,
            ProviderKind::Reddit => CONFIG.enable_reddit,
            ProviderKind::Habr => CONFIG.enable_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_prints_arxiv, //todo add for
            ProviderKind::Biorxiv => CONFIG.enable_prints_biorxiv, //todo add for
            ProviderKind::Github => CONFIG.enable_prints_github, //todo add for
            ProviderKind::Medrxiv => CONFIG.enable_prints_medrxiv, //todo add for
            ProviderKind::Twitter => CONFIG.enable_prints_twitter, //todo add for
            ProviderKind::Reddit => CONFIG.enable_prints_reddit, //todo add for
            ProviderKind::Habr => CONFIG.enable_prints_habr,   //todo add for
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_warning_high_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_warning_high_prints_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_warning_high_prints_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_warning_high_prints_for_github,
            ProviderKind::Medrxiv => CONFIG.enable_warning_high_prints_for_medrxiv,
            ProviderKind::Twitter => CONFIG.enable_warning_high_prints_for_twitter,
            ProviderKind::Reddit => CONFIG.enable_warning_high_prints_for_reddit,
            ProviderKind::Habr => CONFIG.enable_warning_high_prints_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_warning_low_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_warning_low_prints_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_warning_low_prints_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_warning_low_prints_for_github,
            ProviderKind::Medrxiv => CONFIG.enable_warning_low_prints_for_medrxiv,
            ProviderKind::Twitter => CONFIG.enable_warning_low_prints_for_twitter,
            ProviderKind::Reddit => CONFIG.enable_warning_low_prints_for_reddit,
            ProviderKind::Habr => CONFIG.enable_warning_low_prints_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_success_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_success_prints_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_success_prints_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_success_prints_for_github,
            ProviderKind::Medrxiv => CONFIG.enable_success_prints_for_medrxiv,
            ProviderKind::Twitter => CONFIG.enable_success_prints_for_twitter,
            ProviderKind::Reddit => CONFIG.enable_success_prints_for_reddit,
            ProviderKind::Habr => CONFIG.enable_success_prints_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_partial_success_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_partial_success_prints_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_partial_success_prints_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_partial_success_prints_for_github,
            ProviderKind::Medrxiv => CONFIG.enable_partial_success_prints_for_medrxiv,
            ProviderKind::Twitter => CONFIG.enable_partial_success_prints_for_twitter,
            ProviderKind::Reddit => CONFIG.enable_partial_success_prints_for_reddit,
            ProviderKind::Habr => CONFIG.enable_partial_success_prints_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_error_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_error_prints_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_error_prints_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_error_prints_for_github,
            ProviderKind::Medrxiv => CONFIG.enable_error_prints_for_medrxiv,
            ProviderKind::Twitter => CONFIG.enable_error_prints_for_twitter,
            ProviderKind::Reddit => CONFIG.enable_error_prints_for_reddit,
            ProviderKind::Habr => CONFIG.enable_error_prints_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_time_measurement_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_time_measurement_prints_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_time_measurement_prints_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_time_measurement_prints_for_github,
            ProviderKind::Medrxiv => CONFIG.enable_time_measurement_prints_for_medrxiv,
            ProviderKind::Twitter => CONFIG.enable_time_measurement_prints_for_twitter,
            ProviderKind::Reddit => CONFIG.enable_time_measurement_prints_for_reddit,
            ProviderKind::Habr => CONFIG.enable_time_measurement_prints_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_info_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_info_prints_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_info_prints_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_info_prints_for_github,
            ProviderKind::Medrxiv => CONFIG.enable_info_prints_for_medrxiv,
            ProviderKind::Twitter => CONFIG.enable_info_prints_for_twitter,
            ProviderKind::Reddit => CONFIG.enable_info_prints_for_reddit,
            ProviderKind::Habr => CONFIG.enable_info_prints_for_habr,
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_links_limit_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_links_limit_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_links_limit_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_links_limit_for_github,
            ProviderKind::Medrxiv => CONFIG.enable_links_limit_for_medrxiv,
            ProviderKind::Twitter => CONFIG.enable_links_limit_for_twitter,
            ProviderKind::Reddit => CONFIG.enable_links_limit_for_reddit,
            ProviderKind::Habr => CONFIG.enable_links_limit_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_links_limit(&self) -> i64 {
        match self {
            ProviderKind::Arxiv => CONFIG.links_limit_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.links_limit_for_biorxiv,
            ProviderKind::Github => CONFIG.links_limit_for_github,
            ProviderKind::Habr => CONFIG.links_limit_for_habr,
            ProviderKind::Medrxiv => CONFIG.links_limit_for_medrxiv,
            ProviderKind::Reddit => CONFIG.links_limit_for_reddit,
            ProviderKind::Twitter => CONFIG.links_limit_for_twitter,
        }
    }
    ///

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_item_handle(&self) -> Option<&'static str> {
        match self {
            ProviderKind::Arxiv => Some("</item>"),
            ProviderKind::Biorxiv => Some("</item>"),
            ProviderKind::Github => Some("</entry>"),
            ProviderKind::Habr => Some("</item>"),
            ProviderKind::Medrxiv => Some("</item>"),
            ProviderKind::Reddit => None,
            ProviderKind::Twitter => Some("</item>"),
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_mongo_doc_randomization_aggregation(&self) -> Option<Document> {
        if CONFIG.enable_links_limit {
            if self.is_mongo_link_parts_randomize_order_enabled() {
                Some(doc! { "$sample" : {"size": self.get_links_limit() }})
            } else {
                Some(doc! { "$limit" : self.get_links_limit() })
            }
        } else {
            None
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_mongo_log_collection_name(&self) -> String {
        format!(
            "{}{}",
            self,
            CONFIG.mongo_providers_logs_db_collection_handle_second_part //todo rename it into db log collection
        )
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_path_to_logs_directory(&self) -> String {
        format!("logs/{}/{:?}", &CONFIG.warning_logs_directory_name, self)
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_path_to_provider_log_file(&self) -> String {
        format!(
            "logs/{}/{:?}/{}",
            &CONFIG.warning_logs_directory_name,
            self,
            &CONFIG.unhandled_success_handled_success_are_there_items_initialized_posts_dir
        )
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_init_local_data_file_path(&self) -> String {
        format!(
            "{}{}_link_parts{}",
            CONFIG.path_to_provider_link_parts_folder, self, CONFIG.log_file_extension
        )
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn remove_logs_directory(&self) -> Result<(), CleanLogsDirError> {
        let path = self.get_path_to_logs_directory();
        if !Path::new(&path).is_dir() {
            return Err(CleanLogsDirError::PathIsNotDir { path });
        }
        fs::remove_dir_all(&path)?;
        Ok(())
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn stringify(&self) -> &'static str {
        match self {
            ProviderKind::Arxiv => stringify!(ProviderKind::Arxiv),
            ProviderKind::Biorxiv => stringify!(ProviderKind::Biorxiv),
            ProviderKind::Github => stringify!(ProviderKind::Github),
            ProviderKind::Habr => stringify!(ProviderKind::Habr),
            ProviderKind::Medrxiv => stringify!(ProviderKind::Medrxiv),
            ProviderKind::Reddit => stringify!(ProviderKind::Reddit),
            ProviderKind::Twitter => stringify!(ProviderKind::Twitter),
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn generate_provider_links(&self, names_vector: Vec<String>) -> Vec<String> {
        match self {
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
    fn generate_hashmap_with_empty_string_vecs_for_enabled_providers(
    ) -> HashMap<ProviderKind, Vec<String>> {
        let mut hashmap_with_empty_vecs = HashMap::<ProviderKind, Vec<String>>::with_capacity(
            ProviderKind::get_enabled_providers_vec().len(),
        );
        for provider_kind in ProviderKind::get_enabled_providers_vec().iter() {
            hashmap_with_empty_vecs.insert(*provider_kind, Vec::<String>::new());
        }
        hashmap_with_empty_vecs
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_enabled_providers_vec() -> Vec<ProviderKind> {
        let mut providers_vec: Vec<ProviderKind> = Vec::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter().filter(|provider_kind| provider_kind.is_enabled())
        {
            providers_vec.push(provider_kind);
        }
        providers_vec
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_enabled_string_name_vec() -> Vec<String> {
        let mut string_name_vec: Vec<String> = Vec::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter().filter(|element| element.is_enabled()) {
            string_name_vec.push(format!("{}", provider_kind));
        }
        string_name_vec
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_mongo_initialization_provider_kind_vec() -> Vec<ProviderKind> {
        let mut vec_of_filtered_provider_names: Vec<ProviderKind> =
            Vec::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter()
            .filter(|provider_kind| provider_kind.is_mongo_initialization_enabled())
        {
            vec_of_filtered_provider_names.push(provider_kind)
        }
        vec_of_filtered_provider_names
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_mongo_initialization_string_name_vec() -> Vec<String> {
        let mut vec_of_filtered_provider_names: Vec<String> =
            Vec::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter()
            .filter(|provider_kind| provider_kind.is_mongo_initialization_enabled())
        {
            vec_of_filtered_provider_names.push(format!("{}", provider_kind))
        }
        vec_of_filtered_provider_names
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn into_string_name_and_kind_hashmap() -> HashMap<String, ProviderKind> {
        //its String coz legacy
        let mut config_provider_string_to_enum_struct_hasmap: HashMap<String, ProviderKind> =
            HashMap::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter() {
            config_provider_string_to_enum_struct_hasmap
                .insert(format!("{}", provider_kind), provider_kind);
        }
        config_provider_string_to_enum_struct_hasmap
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn into_string_name_and_kind_tuple_vec() -> Vec<(String, ProviderKind)> {
        let mut provider_kind_vec = Vec::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter() {
            provider_kind_vec.push((format!("{}", provider_kind), provider_kind));
        }
        provider_kind_vec
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn remove_existing_providers_logs_directories(
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
    fn remove_providers_logs_directories() -> Result<(), HashMap<ProviderKind, CleanLogsDirError>> {
        let mut result_hashmap: HashMap<ProviderKind, CleanLogsDirError> =
            HashMap::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter()
            .filter(|provider_kind| provider_kind.is_cleaning_warning_logs_directory_enabled())
        {
            if let Err(e) = provider_kind.remove_logs_directory() {
                result_hashmap.insert(provider_kind, e);
            }
        }
        if result_hashmap.is_empty() {
            Ok(())
        } else {
            Err(result_hashmap)
        }
    }
}
