use crate::helpers::json_to_string::json_to_string;

use crate::fetch::provider_log_into_json::provider_log_into_json;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;
use crate::logs_logic::drop_mongo_provider_logs_collection_if_need::drop_mongo_provider_logs_collection_if_need;

use config_lib::get_project_information::get_config::get_lazy_config_information::CONFIG;
use config_lib::get_project_information::project_constants::get_mongo_url;
use config_lib::get_project_information::provider_kind_enum::ProviderKind;

use mongo_integration::mongo_drop_db::mongo_drop_db;
use mongo_integration::mongo_insert_docs_in_empty_collection::mongo_insert_docs_in_empty_collection;

use std::time::Instant;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

use std::collections::HashMap;

use futures::future::join_all;

#[derive(
    Clone, Debug, serde_derive::Serialize, serde_derive::Deserialize, PartialEq, Eq, Hash, Copy,
)]
pub enum WriteLogsResult {
    Success,
    PartialSuccess, //todo: add values in here
    Failure,
}

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
) -> WriteLogsResult {
    if !error_posts.is_empty() {
        let time = Instant::now();
        let mongo_url = get_mongo_url();
        //todo: move this to config
        let db_name_handle = "logs";
        let db_collection_handle_second_part = "second_part";
        let db_collection_document_field_name_handle = "data";
        let mut vec_of_error_provider_kinds: Vec<ProviderKind> =
            Vec::with_capacity(error_posts.len());
        let mut hashmap_of_provider_vec_of_strings: HashMap<ProviderKind, Vec<String>> =
            HashMap::new();
        //restructure into hashmap for better usage
        for element in &error_posts {
            if !vec_of_error_provider_kinds.contains(&element.4) {
                let empty_vec_of_stringified_json: Vec<String> = Vec::new();
                hashmap_of_provider_vec_of_strings.insert(element.4, empty_vec_of_stringified_json);
                vec_of_error_provider_kinds.push(element.4);
            }
        }
        let hashmap_len = hashmap_of_provider_vec_of_strings.len();
        if CONFIG.params.enable_cleaning_warning_logs_db_in_mongo {
            /////////////////////////////////////////////////////
            //this error exists only for cloud mongo
            //file: libs/mongo_integration/src/mongo_drop_db.rs:25
            // Drop failed Error {
            //     kind: CommandError(
            //         CommandError {
            //             code: 8000,
            //             code_name: "AtlasError",
            //             message: "user is not allowed to do action [dropDatabase] on [logs.]",
            //             labels: [],
            //         },
            //     ),
            //     labels: [],
            // }
            /////////////////////////////////////////////////////
            //old tokio runtime
            let dropping_db_result = mongo_drop_db(&mongo_url, db_name_handle);
            match dropping_db_result {
                Ok(_) => (),
                Err(e) => {
                    print_colorful_message(
                    None,
                    PrintType::Error,
                    file!().to_string(),
                    line!().to_string(),
                    format!(
                        "mongo_drop_db url: {}, db name: {}, error: {:#?} \n maybe disable mongo dropping db parameter in config?",
                        &mongo_url, db_name_handle, e
                    ),
                );
                    return WriteLogsResult::Failure;
                }
            }
        }
        let mut vec_of_failed_collections_drops: Vec<ProviderKind> =
            Vec::with_capacity(vec_of_error_provider_kinds.len());
        if CONFIG
            .params
            .enable_cleaning_warning_logs_db_collections_in_mongo
        {
            let mut vec_join = Vec::new();
            for provider_kind_handle in vec_of_error_provider_kinds {
                let mongo_url_clone = mongo_url.clone();
                vec_join.push(drop_mongo_logs_collection_wrapper_for_providers(
                    provider_kind_handle,
                    mongo_url_clone,
                    db_collection_handle_second_part,
                    db_name_handle,
                ))
            }
            let result_vec = join_all(vec_join).await;
            vec_of_failed_collections_drops = result_vec
                .into_iter()
                .filter(|x| !x.1)
                .map(|x: (ProviderKind, bool)| -> ProviderKind { x.0 })
                .collect();
        }
        for (
            link,
            unhandled_fetch_status_info,
            handled_fetch_status_info,
            are_there_items,
            provider_kind,
        ) in error_posts
        {
            if !vec_of_failed_collections_drops.contains(&provider_kind) {
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
                            Some(stringified_json_vec) => {
                                stringified_json_vec.push(stringified_json)
                            }
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
        }
        let mut vec_of_futures = Vec::with_capacity(hashmap_of_provider_vec_of_strings.len());
        for element in hashmap_of_provider_vec_of_strings {
            let collection_handle = format!(
                "{:#?}{}",
                element.0.clone(),
                db_collection_handle_second_part
            );
            //if push mongo_insert_docs_in_empty_collection then cant do join_all()
            vec_of_futures.push(
                mongo_insert_docs_in_empty_collection_wrapper_under_old_tokio_version(
                    element.0,
                    &mongo_url,
                    db_name_handle,
                    collection_handle, //fix naming later
                    db_collection_document_field_name_handle,
                    element.1,
                ),
            );
        }
        //todo write some logic around it
        let results_vec = join_all(vec_of_futures).await;
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
        let results_vec_len = results_vec.len();
        if results_vec_len == hashmap_len {
            // v.into_iter().reduce(|a, b| a + b);
            let mut checker_if_all_true = true;
            for (_, boolean_result) in &results_vec {
                if !(*boolean_result) {
                    checker_if_all_true = false;
                    break;
                }
            }
            if checker_if_all_true {
                WriteLogsResult::Success
            } else {
                let mut checker_if_all_false = true;
                for (_, boolean_result) in results_vec {
                    if boolean_result {
                        checker_if_all_false = false;
                        break;
                    }
                }
                if checker_if_all_false {
                    WriteLogsResult::Failure
                } else {
                    //todo write more info about this case
                    print_colorful_message(
                        None,
                        PrintType::WarningLow,
                        file!().to_string(),
                        line!().to_string(),
                        "partial_success coz results_vec not all true".to_string(),
                    );
                    WriteLogsResult::PartialSuccess
                }
            }
        } else {
            let mut checker_if_all_false = true;
            for (_, boolean_result) in results_vec {
                if boolean_result {
                    checker_if_all_false = false;
                    break;
                }
            }
            if checker_if_all_false {
                WriteLogsResult::Failure
            } else {
                //todo write more info about this case
                print_colorful_message(
                    None,
                    PrintType::WarningLow,
                    file!().to_string(),
                    line!().to_string(),
                    format!("partial_success coz results_vec_len != hashmap_len and results_vec not all true, results_vec.len() {}, hashmap_len {}",
                               results_vec_len, hashmap_len),
                );
                WriteLogsResult::PartialSuccess
            }
        }
    } else {
        print_colorful_message(
            None,
            PrintType::WarningLow,
            file!().to_string(),
            line!().to_string(),
            "error_posts.len() == 0".to_string(),
        );
        WriteLogsResult::Success
    }
}

//this function was created to have ability do join_all()
pub async fn mongo_insert_docs_in_empty_collection_wrapper_under_old_tokio_version(
    provider_kind: ProviderKind,
    mongo_url: &str,
    db_name_handle: &str,
    db_collection_handle: String,
    db_collection_document_field_name_handle: &str,
    vec_of_values: Vec<String>,
) -> (ProviderKind, bool) {
    let vec_of_values_len = vec_of_values.len();
    //old tokio runtime
    let result = mongo_insert_docs_in_empty_collection(
        &mongo_url,
        db_name_handle,
        db_collection_handle, //fix naming later
        db_collection_document_field_name_handle,
        vec_of_values,
    );
    match result {
        Ok(boolean_result) => {
            print_colorful_message(
                None,
                PrintType::Success,
                file!().to_string(),
                line!().to_string(),
                format!(
                    "successfull insertion {} elements into {:#?}",
                    vec_of_values_len, provider_kind
                ),
            );
            return (provider_kind, boolean_result);
        }
        Err(e) => {
            print_colorful_message(
                None,
                PrintType::WarningHigh,
                file!().to_string(),
                line!().to_string(),
                format!("mongo_insert_docs_in_empty_collection error {:#?}", e),
            );
            return (provider_kind, false);
        }
    }
}

pub async fn drop_mongo_logs_collection_wrapper_for_providers(
    provider_kind_handle: ProviderKind,
    mongo_url: String,
    db_collection_handle_second_part_clone: &str,
    db_name_handle: &str,
) -> (ProviderKind, bool) {
    let result_of_dropping_collection: (ProviderKind, bool);
    match provider_kind_handle {
        ProviderKind::Arxiv => {
            result_of_dropping_collection = drop_mongo_provider_logs_collection_if_need(
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_arxiv,
                provider_kind_handle,
                db_collection_handle_second_part_clone.to_string(),
                mongo_url,
                db_name_handle.to_string(),
            )
            .await;
        }
        ProviderKind::Biorxiv => {
            result_of_dropping_collection = drop_mongo_provider_logs_collection_if_need(
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_biorxiv,
                provider_kind_handle,
                db_collection_handle_second_part_clone.to_string(),
                mongo_url,
                db_name_handle.to_string(),
            )
            .await;
        }
        ProviderKind::Github => {
            result_of_dropping_collection = drop_mongo_provider_logs_collection_if_need(
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_github,
                provider_kind_handle,
                db_collection_handle_second_part_clone.to_string(),
                mongo_url,
                db_name_handle.to_string(),
            )
            .await;
        }
        ProviderKind::Habr => {
            result_of_dropping_collection = drop_mongo_provider_logs_collection_if_need(
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_habr,
                provider_kind_handle,
                db_collection_handle_second_part_clone.to_string(),
                mongo_url,
                db_name_handle.to_string(),
            )
            .await;
        }
        ProviderKind::Medrxiv => {
            result_of_dropping_collection = drop_mongo_provider_logs_collection_if_need(
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_medrxiv,
                provider_kind_handle,
                db_collection_handle_second_part_clone.to_string(),
                mongo_url,
                db_name_handle.to_string(),
            )
            .await;
        }
        ProviderKind::Reddit => {
            result_of_dropping_collection = drop_mongo_provider_logs_collection_if_need(
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_reddit,
                provider_kind_handle,
                db_collection_handle_second_part_clone.to_string(),
                mongo_url,
                db_name_handle.to_string(),
            )
            .await;
        }
        ProviderKind::Twitter => {
            result_of_dropping_collection = drop_mongo_provider_logs_collection_if_need(
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_twitter,
                provider_kind_handle,
                db_collection_handle_second_part_clone.to_string(),
                mongo_url,
                db_name_handle.to_string(),
            )
            .await;
        }
    };
    result_of_dropping_collection
}
