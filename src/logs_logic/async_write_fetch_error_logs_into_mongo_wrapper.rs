use crate::fetch::error_logs_into_json::error_logs_into_json;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;
use config_lib::get_project_information::provider_kind_enum::ProviderKind;
use std::time::Instant;

//under development
use config_lib::get_project_information::get_config::get_lazy_config_information::CONFIG;
use config_lib::get_project_information::get_user_credentials::get_lazy_user_credentials_information::USER_CREDENTIALS;
use mongo_integration::mongo_drop_collection_wrapper::mongo_drop_collection_wrapper;
use mongo_integration::mongo_insert_docs_in_empty_collection::mongo_insert_docs_in_empty_collection;
//under development

use serde_json::json;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn async_write_fetch_error_logs_into_mongo_wrapper(
    error_posts: Vec<(
        String,
        UnhandledFetchStatusInfo,
        HandledFetchStatusInfo,
        AreThereItems,
        ProviderKind,
    )>,
) {
    let time = Instant::now();
    ////////////////////////////////
    let mut vec_of_json = Vec::with_capacity(error_posts.len());
    for (
        link,
        unhandled_fetch_status_info,
        handled_fetch_status_info,
        are_there_items,
        provider_kind,
    ) in error_posts
    {
        let option_json = error_logs_into_json(
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
    ////////////////////////////////

    //todo write into mongo collection and create flag where to write logs
    let mongo_cloud_first_handle_url_part = &CONFIG.mongo_params.mongo_cloud_first_handle_url_part;
    let mongo_cloud_login = &USER_CREDENTIALS.mongo_cloud_authorization.mongo_cloud_login;
    let mongo_cloud_second_handle_url_part =
        &CONFIG.mongo_params.mongo_cloud_second_handle_url_part;
    let mongo_cloud_password = &USER_CREDENTIALS
        .mongo_cloud_authorization
        .mongo_cloud_password;
    let mongo_cloud_third_handle_url_part = &CONFIG.mongo_params.mongo_cloud_third_handle_url_part;
    let mongo_cloud_cluster_name = &USER_CREDENTIALS
        .mongo_cloud_authorization
        .mongo_cloud_cluster_name;
    let mongo_cloud_fourth_handle_url_part =
        &CONFIG.mongo_params.mongo_cloud_fourth_handle_url_part;
    let mongo_cloud_cluster_params = &USER_CREDENTIALS
        .mongo_cloud_authorization
        .mongo_cloud_cluster_params;
    let mongo_url = format!(
        "{}{}{}{}{}{}{}{}",
        mongo_cloud_first_handle_url_part,
        mongo_cloud_login,
        mongo_cloud_second_handle_url_part,
        mongo_cloud_password,
        mongo_cloud_third_handle_url_part,
        mongo_cloud_cluster_name,
        mongo_cloud_fourth_handle_url_part,
        mongo_cloud_cluster_params
    );
    let db_name_handle = "logs";
    let db_collection_name = "arxiv";
    let check_if_collection_empty = false;
    let db_collection_document_field_name_handle = "data";
    let key = "key";
    let db_collection_handle_second_part = "second_part";
    let vec_of_link_parts: Vec<String> = vec!["f".to_string()];
    // let json_object = json!({
    //                     "link": &value.0,
    //                     "part_of": format!("{:?}", value.4),
    //                     "date": Local::now().to_string()
    //                 });
    ////////////////////////////////
    //here only one insert. Need many
    ////////////////////////////////
    ////////
    let future_possible_drop_collection = mongo_drop_collection_wrapper(
        &mongo_url,
        db_name_handle,
        &format!("{}{}", key, db_collection_handle_second_part),
        false,
    );
    match future_possible_drop_collection {
        Ok(result_flag) => {
            if result_flag {
                println!("drop done!");
            } else {
                println!("drop fail with flag");
            }
        }
        Err(e) => {
            println!("drop fail with error {:#?}", e);
        }
    }
    //////////////////////////////////////////////////////////////////////////////////////////////
    let future_inserting_docs = mongo_insert_docs_in_empty_collection(
        &mongo_url,
        db_name_handle,
        &format!("{}{}", key, db_collection_handle_second_part),
        db_collection_document_field_name_handle,
        vec_of_link_parts,
    );
    let mut result_flag = true;
    //     thread '<unnamed>' panicked at 'cannot execute `LocalPool` executor from within another executor: EnterError', /home/sergey/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-executor-0.3.14/src/local_pool.rs:78:26
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    match future_inserting_docs {
        Ok(_) => {
            println!("SUCCEEES")
        }
        Err(e) => {
            result_flag = false;
            println!("future_inserting_docs error {:#?}", e);
        }
    }
    if CONFIG.params.enable_time_measurement_prints {
        println!(
            "write fetch error logs into files done in {} seconds {} miliseconds",
            time.elapsed().as_secs(),
            time.elapsed().as_millis(),
        );
    }
}
