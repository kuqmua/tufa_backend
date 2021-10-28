use std::thread;

use crate::check_new_posts_threads_parts::check_new_posts_threads_parts;

use crate::fetch::rss_async_write_fetch_error_logs_into_files_wrapper::rss_async_write_fetch_error_logs_into_files_wrapper;
use crate::logs_logic::async_write_fetch_error_logs_into_mongo_wrapper::async_write_fetch_error_logs_into_mongo_wrapper;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::config_mods::config::CONFIG;

use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;
use crate::mongo_integration::mongo_insert_data::mongo_insert_data;

#[deny(clippy::indexing_slicing)]
#[tokio::main]
pub async fn async_tokio_wrapper() {
    //todo: add check of doc already is in collection or add flag forse
    //todo add flag for provider
    let _ = mongo_insert_data(
        &mongo_get_db_url(),
        &CONFIG.mongo_params.providers_db_name_handle
    )
    .await;
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
