use std::thread;

use crate::check_new_posts_threads_parts::check_new_posts_threads_parts;

use crate::fetch::rss_async_write_fetch_error_logs_into_files_wrapper::rss_async_write_fetch_error_logs_into_files_wrapper;
use crate::logs_logic::async_write_fetch_error_logs_into_mongo_wrapper::async_write_fetch_error_logs_into_mongo_wrapper;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::config_mods::config::CONFIG;

use crate::constants::project_constants::ARXIV_NAME_TO_CHECK;
use crate::constants::project_constants::BIORXIV_NAME_TO_CHECK;
use crate::constants::project_constants::GITHUB_NAME_TO_CHECK;
use crate::constants::project_constants::HABR_NAME_TO_CHECK;
use crate::constants::project_constants::MEDRXIV_NAME_TO_CHECK;
use crate::constants::project_constants::REDDIT_NAME_TO_CHECK;
use crate::constants::project_constants::TWITTER_NAME_TO_CHECK;
use crate::providers::provider_kind_enum::ProviderKind;

use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;
use crate::providers::init_mongo_db_and_collections::put_data_in_mongo::put_data_in_mongo;

#[deny(clippy::indexing_slicing)]
#[tokio::main]
pub async fn async_tokio_wrapper() {
    /////
    let s = ProviderKind::get_length();
    let b = ProviderKind::into_vec();
    println!("gggg {}", s);
    println!("bbbb {:?}", b);
    let veeeec = ProviderKind::get_provider_kind_array_from_string_vec(
        CONFIG.params.vec_of_provider_names.clone(),
    );
    println!("veeeec {:?}", veeeec);
    let mut vec_of_filtered_provider_names: Vec<String> =
        Vec::with_capacity(CONFIG.params.vec_of_provider_names.len());
    //todo rewrite it with type system check help. right now its a bad way to check
    for provider_name in &CONFIG.params.vec_of_provider_names {
        if (provider_name == ARXIV_NAME_TO_CHECK
            && CONFIG
                .mongo_params
                .enable_initialize_mongo_with_providers_link_parts
                .enable_initialize_mongo_with_arxiv_link_parts)
            || (provider_name == BIORXIV_NAME_TO_CHECK
                && CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_biorxiv_link_parts)
            || (provider_name == GITHUB_NAME_TO_CHECK
                && CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_github_link_parts)
            || (provider_name == HABR_NAME_TO_CHECK
                && CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_habr_link_parts)
            || (provider_name == MEDRXIV_NAME_TO_CHECK
                && CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_medrxiv_link_parts)
            || (provider_name == REDDIT_NAME_TO_CHECK
                && CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_reddit_link_parts)
            || (provider_name == TWITTER_NAME_TO_CHECK
                && CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_twitter_link_parts)
        {
            vec_of_filtered_provider_names.push(provider_name.to_string());
        }
    }
    //todo: add check of doc already is in collection or add flag forse
    //todo add flag for provider
    let _ = put_data_in_mongo(
        &mongo_get_db_url(),
        &CONFIG.mongo_params.providers_db_name_handle,
        &CONFIG
            .mongo_params
            .providers_db_collection_handle_second_part,
        &CONFIG
            .mongo_params
            .providers_db_collection_document_field_name_handle,
        &CONFIG.mongo_params.path_to_provider_link_parts_folder,
        // CONFIG.params.vec_of_provider_names.clone(),
        vec_of_filtered_provider_names,
        &CONFIG.mongo_params.log_file_extension,
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
