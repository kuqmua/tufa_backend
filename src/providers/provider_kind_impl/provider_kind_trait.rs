use std::collections::HashMap;
use std::fs;
use std::path::Path;

use mongodb::bson::{doc, Document};
use strum::IntoEnumIterator;

use crate::traits::enum_extention::EnumExtenstion;

use crate::config_mods::lazy_static_config::CONFIG;

use crate::traits::provider_kind_from_config_trait::ProviderKindFromConfigTrait;
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
        if CONFIG.is_links_limit_enabled {
            if self.is_mongo_link_parts_randomize_order_enabled() {
                Some(doc! { "$sample" : {"size": self.links_limit() }})
            } else {
                Some(doc! { "$limit" : self.links_limit() })
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

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_db_tag(&self) -> String {
        format!("{}", self)
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_postgres_table_name(&self) -> String {
        format!("{}_link_parts", self.to_lower_snake_case())
    }
}
