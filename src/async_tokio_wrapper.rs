use std::thread;

use crate::check_new_posts_threads_parts::check_new_posts_threads_parts;

use crate::fetch::rss_async_write_fetch_error_logs_into_files_wrapper::rss_async_write_fetch_error_logs_into_files_wrapper;
use crate::logs_logic::async_write_fetch_error_logs_into_mongo_wrapper::async_write_fetch_error_logs_into_mongo_wrapper;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

use config_lib::get_project_information::get_config::get_lazy_config_information::CONFIG;
use config_lib::get_project_information::get_user_credentials::get_lazy_user_credentials_information::USER_CREDENTIALS;
use providers_info_lib::init_mongo_db_and_collections::put_data_in_mongo::put_data_in_mongo;

#[deny(clippy::indexing_slicing)]
#[tokio::main]
pub async fn async_tokio_wrapper() {
    if CONFIG
        .params
        .enable_initialize_mongo_with_providers_link_parts
    {
        let mongo_url: String;
        if CONFIG.mongo_params.is_cloud {
            let mongo_cloud_first_handle_url_part =
                &CONFIG.mongo_params.mongo_cloud_first_handle_url_part;
            let mongo_cloud_login = &USER_CREDENTIALS.mongo_cloud_authorization.mongo_cloud_login;
            let mongo_cloud_second_handle_url_part =
                &CONFIG.mongo_params.mongo_cloud_second_handle_url_part;
            let mongo_cloud_password = &USER_CREDENTIALS
                .mongo_cloud_authorization
                .mongo_cloud_password;
            let mongo_cloud_third_handle_url_part =
                &CONFIG.mongo_params.mongo_cloud_third_handle_url_part;
            let mongo_cloud_cluster_name = &USER_CREDENTIALS
                .mongo_cloud_authorization
                .mongo_cloud_cluster_name;
            let mongo_cloud_fourth_handle_url_part =
                &CONFIG.mongo_params.mongo_cloud_fourth_handle_url_part;
            let mongo_cloud_cluster_params = &USER_CREDENTIALS
                .mongo_cloud_authorization
                .mongo_cloud_cluster_params;
            mongo_url = format!(
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
        } else {
            let mongo_own_first_handle_url_part =
                &CONFIG.mongo_params.mongo_own_first_handle_url_part;
            let mongo_own_login = &USER_CREDENTIALS.mongo_own_authorization.mongo_own_login;
            let mongo_own_second_handle_url_part =
                &CONFIG.mongo_params.mongo_own_second_handle_url_part;
            let mongo_own_password = &USER_CREDENTIALS.mongo_own_authorization.mongo_own_password;
            let mongo_own_third_handle_url_part =
                &CONFIG.mongo_params.mongo_own_third_handle_url_part;
            let mongo_own_ip = &USER_CREDENTIALS.mongo_own_authorization.mongo_own_ip;
            let mongo_own_fourth_handle_url_part =
                &CONFIG.mongo_params.mongo_own_fourth_handle_url_part;
            let mongo_own_port = &USER_CREDENTIALS.mongo_own_authorization.mongo_own_port;
            mongo_url = format!(
                "{}{}{}{}{}{}{}{}",
                mongo_own_first_handle_url_part,
                mongo_own_login,
                mongo_own_second_handle_url_part,
                mongo_own_password,
                mongo_own_third_handle_url_part,
                mongo_own_ip,
                mongo_own_fourth_handle_url_part,
                mongo_own_port
            );
        }
        //todo: add check of doc already is in collection or add flag forse
        let _ = put_data_in_mongo(
            &mongo_url,
            &CONFIG.mongo_params.providers_db_name_handle,
            &CONFIG
                .mongo_params
                .providers_db_collection_handle_second_part,
            &CONFIG
                .mongo_params
                .providers_db_collection_document_field_name_handle,
            &CONFIG.mongo_params.path_to_provider_link_parts_folder,
            CONFIG.params.vec_of_provider_names.clone(),
            &CONFIG.mongo_params.log_file_extension,
        )
        .await;
    }
    let option_tuple = check_new_posts_threads_parts().await;

    match option_tuple {
        Some((_posts, error_posts)) => {
            if !error_posts.is_empty() {
                let wrong_cases_thread = thread::spawn(move || {
                    // println!("error_posts_done_len{:#?}", error_posts);
                    //todo add flag in config or if its already exists put it here
                    // pub enable_initialize_mongo_with_providers_link_parts: bool,
                    if CONFIG.params.enable_write_error_logs_in_local_folder
                        && CONFIG.params.enable_write_error_logs_in_mongo
                    {
                        async_write_fetch_error_logs_into_mongo_wrapper(error_posts.clone());
                        rss_async_write_fetch_error_logs_into_files_wrapper(error_posts);
                    } else if CONFIG.params.enable_write_error_logs_in_local_folder {
                        async_write_fetch_error_logs_into_mongo_wrapper(error_posts);
                    } else if CONFIG.params.enable_write_error_logs_in_mongo {
                        rss_async_write_fetch_error_logs_into_files_wrapper(error_posts);
                    }
                });
                match wrong_cases_thread.join() {
                    Ok(_) => {}
                    Err(e) => {
                        print_colorful_message(
                            None,
                            PrintType::Error,
                            file!().to_string(),
                            line!().to_string(),
                            format!("wrong_cases_thread.join() error: {:#?}", e),
                        );
                    }
                }
            }
        }
        None => {
            print_colorful_message(
                None,
                PrintType::WarningLow,
                file!().to_string(),
                line!().to_string(),
                "check_new_posts_threads_parts().await - no new posts".to_string(),
            );
        }
    }
}
