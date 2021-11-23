use std::{collections::HashMap, path::Path};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client,
};

use std::fs;

use std::sync::{Arc, Mutex};

use futures::future::join_all;

use crate::config_mods::config::CONFIG;
use crate::constants::project_constants::ARXIV_NAME_TO_CHECK;
use crate::constants::project_constants::BIORXIV_NAME_TO_CHECK;
use crate::constants::project_constants::GITHUB_NAME_TO_CHECK;
use crate::constants::project_constants::HABR_NAME_TO_CHECK;
use crate::constants::project_constants::MEDRXIV_NAME_TO_CHECK;
use crate::constants::project_constants::REDDIT_NAME_TO_CHECK;
use crate::constants::project_constants::TWITTER_NAME_TO_CHECK;

use crate::constants::project_constants::ARXIV_PROVIDER_ITEM_HANDLE;
use crate::constants::project_constants::BIORXIV_PROVIDER_ITEM_HANDLE;
use crate::constants::project_constants::GITHUB_PROVIDER_ITEM_HANDLE;
use crate::constants::project_constants::HABR_PROVIDER_ITEM_HANDLE;
use crate::constants::project_constants::MEDRXIV_PROVIDER_ITEM_HANDLE;
use crate::constants::project_constants::TWITTER_PROVIDER_ITEM_HANDLE;

use procedural_macros_lib::EnumVariantCount;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;
use crate::mongo_integration::mongo_get_documents_as_string_vector::mongo_get_documents_as_string_vector;

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
    //todo: collection logs or what? there are many collections...
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
    pub fn is_prints_enabled(provider_kind: ProviderKind) -> bool {
        match provider_kind {
            ProviderKind::Arxiv => CONFIG.enable_providers_prints.enable_prints_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_providers_prints.enable_prints_biorxiv,
            ProviderKind::Github => CONFIG.enable_providers_prints.enable_prints_github,
            ProviderKind::Habr => CONFIG.enable_providers_prints.enable_prints_habr,
            ProviderKind::Medrxiv => CONFIG.enable_providers_prints.enable_prints_medrxiv,
            ProviderKind::Reddit => CONFIG.enable_providers_prints.enable_prints_reddit,
            ProviderKind::Twitter => CONFIG.enable_providers_prints.enable_prints_twitter,
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
    pub fn is_cleaning_warning_logs_db_collections_in_mongo_enabled(
        provider_kind: ProviderKind,
    ) -> bool {
        match provider_kind {
            ProviderKind::Arxiv => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_arxiv
            }
            ProviderKind::Biorxiv => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_biorxiv
            }
            ProviderKind::Github => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_github
            }
            ProviderKind::Habr => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_habr
            }
            ProviderKind::Medrxiv => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_medrxiv
            }
            ProviderKind::Reddit => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_reddit
            }
            ProviderKind::Twitter => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_twitter
            }
        }
    }
    pub fn stringify(provider_kind: ProviderKind) -> &'static str {
        match provider_kind {
            ProviderKind::Arxiv => stringify!(ProviderKind::Arxiv),
            ProviderKind::Biorxiv => stringify!(ProviderKind::Biorxiv),
            ProviderKind::Github => stringify!(ProviderKind::Github),
            ProviderKind::Habr => stringify!(ProviderKind::Habr),
            ProviderKind::Medrxiv => stringify!(ProviderKind::Medrxiv),
            ProviderKind::Reddit => stringify!(ProviderKind::Reddit),
            ProviderKind::Twitter => stringify!(ProviderKind::Twitter),
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
                let vec_of_strings = mongo_get_documents_as_string_vector(
                    collection,
                    &CONFIG
                        .mongo_params
                        .providers_db_collection_document_field_name_handle,
                    option_aggregation_stage_1_get_docs_in_random_order_with_limit,
                )
                .await?;
                //todo remove option
                return Ok(Some(vec_of_strings));
            }
        }
        Ok(None)
    }
    #[deny(clippy::indexing_slicing)] //, clippy::unwrap_used
    pub async fn mongo_get_providers_link_parts_unprocessed() -> Result<
        HashMap<ProviderKind, Result<Vec<String>, mongodb::error::Error>>,
        mongodb::error::Error,
    > {
        let client_options = ClientOptions::parse(mongo_get_db_url()).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(&CONFIG.mongo_params.providers_db_name_handle);
        let vec_collection_names = db.list_collection_names(None).await?;
        let vec_provider_kind_with_collection_names_under_arc = Arc::new(Mutex::new(HashMap::<
            ProviderKind,
            Result<Vec<String>, mongodb::error::Error>,
        >::new()
    ));
        let mut vec_of_tasks = Vec::with_capacity(ProviderKind::get_enabled_providers_vec().len());
        for provider_kind in ProviderKind::get_enabled_providers_vec() {
            let vec_provider_kind_with_collection_names_under_arc_handle =
                Arc::clone(&vec_provider_kind_with_collection_names_under_arc);
            let collection_name = ProviderKind::get_mongo_collection_name(provider_kind);
            let collection = db.collection::<Document>(&collection_name);
            if vec_collection_names.contains(&collection_name) {
                vec_of_tasks.push(tokio::task::spawn(async move {
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
                                option_aggregation_stage_1_get_docs_in_random_order_with_limit =
                                    Some(doc! { "$limit" :  CONFIG.params.common_providers_links_limit });
                            }
                        } else {
                            option_aggregation_stage_1_get_docs_in_random_order_with_limit =
                                ProviderKind::get_mongo_doc_randomization_aggregation(provider_kind);
                        }
                    } else {
                        option_aggregation_stage_1_get_docs_in_random_order_with_limit = None;
                    }
                    let result_vec_of_strings = mongo_get_documents_as_string_vector(
                        collection,
                        &CONFIG
                            .mongo_params
                            .providers_db_collection_document_field_name_handle,
                        option_aggregation_stage_1_get_docs_in_random_order_with_limit,
                    )
                    .await;
                    let mut vec_provider_kind_with_collection_names_under_arc_handle_locked =
                        vec_provider_kind_with_collection_names_under_arc_handle
                            .lock()
                            .unwrap();
                    match result_vec_of_strings {
                        Ok(vec_of_strings) => {
                            vec_provider_kind_with_collection_names_under_arc_handle_locked
                                .insert(provider_kind, Ok(vec_of_strings));
                        },
                        Err(e) => {
                            vec_provider_kind_with_collection_names_under_arc_handle_locked
                                .insert(provider_kind, Err(e));
                        },
                    }
                }));
            } else {
                let mut vec_provider_kind_with_collection_names_under_arc_handle_locked =
                    vec_provider_kind_with_collection_names_under_arc_handle
                        .lock()
                        .unwrap();
                vec_provider_kind_with_collection_names_under_arc_handle_locked
                    .insert(provider_kind, Ok(Vec::<String>::new()));
            }
        }
        let _ = join_all(vec_of_tasks).await;
        let vec_provider_kind_with_collection_names =
            vec_provider_kind_with_collection_names_under_arc
                .lock()
                .unwrap()
                .clone();
        Ok(vec_provider_kind_with_collection_names)
    }
    /////////////////
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
