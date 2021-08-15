use crate::helpers::json_to_string::json_to_string;

use crate::fetch::provider_log_into_json::provider_log_into_json;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;
use config_lib::get_project_information::get_config::get_lazy_config_information::CONFIG;
use config_lib::get_project_information::project_constants::get_mongo_url;
use config_lib::get_project_information::provider_kind_enum::ProviderKind;
use mongo_integration::mongo_drop_collection_wrapper::mongo_drop_collection_wrapper;
use mongo_integration::mongo_insert_docs_in_empty_collection::mongo_insert_docs_in_empty_collection;
use std::time::Instant;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

use std::collections::HashMap;

// use futures::future::join_all;

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
    //todo: drop db? or drop collection in loop for unique provider kind

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
    ////////////////////////////////////////////////////////////////////////////////////////////////
    for element in vec_of_error_provider_kinds {
        if true {
            //CONFIG.params.enable_cleaning_warning_logs_directory
            let db_collection_name = &format!("{:#?}{}", element, db_collection_handle_second_part);
            let future_possible_drop_collection = mongo_drop_collection_wrapper(
                &mongo_url,
                db_name_handle,
                db_collection_name,
                false,
            );
            match future_possible_drop_collection {
                Ok(result_flag) => {
                    if result_flag {
                        println!("drop done!");
                    } else {
                        println!("drop fail with flag");
                        return result_flag;
                    }
                }
                Err(e) => {
                    println!("drop fail with error {:#?}", e);
                    return false;
                }
            }
        }
    }
    //////////////////////////////////////////////////
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
                    Some(vecddd) => vecddd.push(stringified_json),
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
