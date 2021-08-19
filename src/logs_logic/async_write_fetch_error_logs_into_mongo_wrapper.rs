use crate::helpers::json_to_string::json_to_string;

use crate::fetch::provider_log_into_json::provider_log_into_json;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;

use config_lib::get_project_information::get_config::get_lazy_config_information::CONFIG;
use config_lib::get_project_information::project_constants::get_mongo_url;
use config_lib::get_project_information::provider_kind_enum::ProviderKind;

use mongo_integration::mongo_drop_collection_wrapper::mongo_drop_collection_wrapper;
use mongo_integration::mongo_drop_db::mongo_drop_db;
use mongo_integration::mongo_insert_docs_in_empty_collection::mongo_insert_docs_in_empty_collection;

use std::time::Instant;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

use std::collections::HashMap;

use futures::future::join_all;

#[deny(clippy::indexing_slicing)] //, clippy::unwrap_used
#[tokio::main]
pub async fn async_write_fetch_error_logs_into_mongo_wrapper(
    error_posts: Vec<(
        String,
        UnhandledFetchStatusInfo,
        HandledFetchStatusInfo,
        AreThereItems,
        ProviderKind,
    )>,
) -> bool {
    let time = Instant::now();
    let mongo_url = get_mongo_url();
    let key = "key"; //must be dynamic: arxiv, biorxiv and etc.
                     //todo: move this to config
    let db_name_handle = "logs";
    let db_collection_handle_second_part = "second_part";
    let db_collection_document_field_name_handle = "data";

    let mut vec_of_error_provider_kinds: Vec<ProviderKind> = Vec::with_capacity(error_posts.len());
    let mut hashmap_of_provider_vec_of_strings: HashMap<ProviderKind, Vec<String>> = HashMap::new();
    for element in &error_posts {
        if !vec_of_error_provider_kinds.contains(&element.4) {
            let empty_vec_of_stringified_json: Vec<String> = Vec::new();
            hashmap_of_provider_vec_of_strings
                .insert(element.4.clone(), empty_vec_of_stringified_json);
            vec_of_error_provider_kinds.push(element.4.clone());
        }
    }
    //todo: drop db? or drop collection in loop for unique provider kind
    //todo: do it in parallel async
    //
    // if CONFIG.params.enable_cleaning_warning_logs_db_in_mongo {
    //     //         file: libs/mongo_integration/src/mongo_drop_db.rs:25
    //     // Drop failed Error {
    //     //     kind: CommandError(
    //     //         CommandError {
    //     //             code: 8000,
    //     //             code_name: "AtlasError",
    //     //             message: "user is not allowed to do action [dropDatabase] on [logs.]",
    //     //             labels: [],
    //     //         },
    //     //     ),
    //     //     labels: [],
    //     // }
    //     let dropping_db_result = mongo_drop_db(&mongo_url, db_name_handle);
    // }
    let mut common_result_of_dropping_collections: bool = false;
    if CONFIG
        .params
        .enable_cleaning_warning_logs_db_collections_in_mongo
    {
        let mut vec_join = Vec::new();
        for provider_kind_handle in vec_of_error_provider_kinds {
            let mongo_url_clone = mongo_url.clone();
            vec_join.push(drop_provider_collection_handle(
                provider_kind_handle,
                mongo_url_clone,
                db_collection_handle_second_part,
                db_name_handle,
            ))
        }
        let result_vec = join_all(vec_join).await;
        let mm: Vec<(ProviderKind, bool)> = result_vec.into_iter().filter(|x| !x.1).collect();
        println!("mm {:#?}", mm.len());
        // for boolean_result in result_vec {

        // }
    }
    let mut vec_of_futures = Vec::with_capacity(hashmap_of_provider_vec_of_strings.len());
    for (
        link,
        unhandled_fetch_status_info,
        handled_fetch_status_info,
        are_there_items,
        provider_kind,
    ) in error_posts
    {
        let option_json = provider_log_into_json(
            &link.clone(), //todo understand lifetimes to remove it
            &unhandled_fetch_status_info,
            &handled_fetch_status_info,
            &are_there_items,
            &provider_kind,
        );
        if let Some(json) = option_json {
            let option_stringified_json = json_to_string(json);
            if let Some(stringified_json) = option_stringified_json {
                match hashmap_of_provider_vec_of_strings.get_mut(&provider_kind) {
                    Some(stringified_json_vec) => stringified_json_vec.push(stringified_json),
                    None => {
                        print_colorful_message(
                            None,
                            PrintType::WarningHigh,
                            file!().to_string(),
                            line!().to_string(),
                            "hashmap_of_provider_vec_of_strings.get_mut(&provider_kind) is None"
                                .to_string(),
                        );
                    }
                }
            }
        }
    }
    for element in hashmap_of_provider_vec_of_strings {
        vec_of_futures.push(mongo_insert_docs_in_empty_collection(
            &mongo_url,
            db_name_handle,
            &format!("{:#?}{}", element.0, db_collection_handle_second_part), //fix naming later
            db_collection_document_field_name_handle,
            element.1,
        ));
    }
    //todo: why cant join all? find out
    // let _ = join_all(vec_of_futures).await;
    if CONFIG.params.enable_time_measurement_prints {
        print_colorful_message(
            None,
            PrintType::TimeMeasurement,
            file!().to_string(),
            line!().to_string(),
            format!(
                "write fetch error logs into files done in {} seconds {} miliseconds",
                time.elapsed().as_secs(),
                time.elapsed().as_millis()
            ),
        );
    };
    true
}

pub async fn drop_provider_collection_handle(
    provider_kind_handle: ProviderKind,
    mongo_url: String,
    db_collection_handle_second_part_clone: &str,
    db_name_handle: &str,
) -> (ProviderKind, bool) {
    let result_of_dropping_collection: (ProviderKind, bool);
    match provider_kind_handle {
        ProviderKind::Arxiv => {
            let result = drop_collection_handle(
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_arxiv,
                provider_kind_handle,
                db_collection_handle_second_part_clone.to_string(),
                mongo_url,
                db_name_handle.to_string(),
            );
            if result {
                result_of_dropping_collection = (provider_kind_handle, true);
            } else {
                result_of_dropping_collection = (provider_kind_handle, false);
            }
        }
        ProviderKind::Biorxiv => {
            let result = drop_collection_handle(
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_biorxiv,
                provider_kind_handle,
                db_collection_handle_second_part_clone.to_string(),
                mongo_url,
                db_name_handle.to_string(),
            );
            if result {
                result_of_dropping_collection = (provider_kind_handle, true);
            } else {
                result_of_dropping_collection = (provider_kind_handle, false);
            }
        }
        ProviderKind::Github => {
            let result = drop_collection_handle(
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_github,
                provider_kind_handle,
                db_collection_handle_second_part_clone.to_string(),
                mongo_url,
                db_name_handle.to_string(),
            );
            if result {
                result_of_dropping_collection = (provider_kind_handle, true);
            } else {
                result_of_dropping_collection = (provider_kind_handle, false);
            }
        }
        ProviderKind::Habr => {
            let result = drop_collection_handle(
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_habr,
                provider_kind_handle,
                db_collection_handle_second_part_clone.to_string(),
                mongo_url,
                db_name_handle.to_string(),
            );
            if result {
                result_of_dropping_collection = (provider_kind_handle, true);
            } else {
                result_of_dropping_collection = (provider_kind_handle, false);
            }
        }
        ProviderKind::Medrxiv => {
            let result = drop_collection_handle(
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_medrxiv,
                provider_kind_handle,
                db_collection_handle_second_part_clone.to_string(),
                mongo_url,
                db_name_handle.to_string(),
            );
            if result {
                result_of_dropping_collection = (provider_kind_handle, true);
            } else {
                result_of_dropping_collection = (provider_kind_handle, false);
            }
        }
        ProviderKind::Reddit => {
            let result = drop_collection_handle(
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_reddit,
                provider_kind_handle,
                db_collection_handle_second_part_clone.to_string(),
                mongo_url,
                db_name_handle.to_string(),
            );
            if result {
                result_of_dropping_collection = (provider_kind_handle, true);
            } else {
                result_of_dropping_collection = (provider_kind_handle, false);
            }
        }
        ProviderKind::Twitter => {
            let result = drop_collection_handle(
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_twitter,
                provider_kind_handle,
                db_collection_handle_second_part_clone.to_string(),
                mongo_url,
                db_name_handle.to_string(),
            );
            if result {
                result_of_dropping_collection = (provider_kind_handle, true);
            } else {
                result_of_dropping_collection = (provider_kind_handle, false);
            }
        }
    };
    result_of_dropping_collection
}

pub fn drop_collection_handle(
    enable_cleaning_warning_logs_db_provider_collection: bool,
    provider_kind_handle: ProviderKind,
    db_collection_handle_second_part: String,
    mongo_url: String,
    db_name_handle: String,
) -> bool {
    let db_collection_handle_second_part_clone = db_collection_handle_second_part.clone();
    let result_flag: bool = true;
    if enable_cleaning_warning_logs_db_provider_collection {
        let db_collection_name = &format!(
            "{:#?}{}",
            provider_kind_handle, db_collection_handle_second_part_clone
        );
        let future_possible_drop_collection = mongo_drop_collection_wrapper(
            &mongo_url,
            &db_name_handle,
            db_collection_name,
            false, //todo
        );
        match future_possible_drop_collection {
            Ok(result_flag) => {
                if !result_flag {
                    println!("drop fail with flag");
                }
                return result_flag;
            }
            Err(e) => {
                println!("drop fail with error {:#?}", e);
                return false;
            }
        }
    }
    result_flag
}
