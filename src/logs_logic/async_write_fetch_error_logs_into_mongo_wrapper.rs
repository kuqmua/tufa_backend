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
    let key = "key";
    let db_name_handle = "logs";
    let db_collection_handle_second_part = "second_part";
    if true {
        //CONFIG.params.enable_cleaning_warning_logs_directory
        let db_collection_name = &format!("{}{}", key, db_collection_handle_second_part);
        let future_possible_drop_collection =
            mongo_drop_collection_wrapper(&mongo_url, db_name_handle, db_collection_name, false);
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
    let db_collection_document_field_name_handle = "data";
    let mut vec_of_json = Vec::with_capacity(error_posts.len());
    let mut vec_of_stringified_json: Vec<String> = Vec::with_capacity(error_posts.len()); //instance here coz use of moved value after for loop
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
        match option_json {
            Some(json_object) => {
                vec_of_json.push(json_object);
            }
            None => {
                print_colorful_message(
                    Some(&provider_kind),
                    PrintType::WarningHigh,
                    file!().to_string(),
                    line!().to_string(),
                    "UnhandledFetchStatusInfo::Success, HandledFetchStatusInfo::Success, AreThereItems::Yep --- its not suppose to happend".to_string(),
                );
            }
        }
    }
    if vec_of_json.is_empty() {
        print_colorful_message(
            None,
            PrintType::WarningLow,
            file!().to_string(),
            line!().to_string(),
            "vec_of_json.is_empty() == true".to_string(),
        );
        return false;
    }
    for json in vec_of_json {
        let option_stringified_json = json_to_string(json);
        if let Some(stringified_json) = option_stringified_json {
            vec_of_stringified_json.push(stringified_json)
        }
    }
    if vec_of_stringified_json.is_empty() {
        print_colorful_message(
            None,
            PrintType::WarningLow,
            file!().to_string(),
            line!().to_string(),
            "vec_of_stringified_json.is_empty() == true".to_string(),
        );
        return false;
    }

    let future_inserting_docs = mongo_insert_docs_in_empty_collection(
        &mongo_url,
        db_name_handle,
        &format!("{}{}", key, db_collection_handle_second_part),
        db_collection_document_field_name_handle,
        vec_of_stringified_json,
    );
    match future_inserting_docs {
        Ok(_) => (),
        Err(e) => {
            print_colorful_message(
                None,
                PrintType::WarningHigh,
                file!().to_string(),
                line!().to_string(),
                format!("future_inserting_docs error {:#?}", e),
            );
            return false;
        }
    }
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
