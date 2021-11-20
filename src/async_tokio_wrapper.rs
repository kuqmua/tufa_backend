use std::thread;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use futures::executor::block_on;

use crate::check_new_posts_threads_parts::check_new_posts_threads_parts;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::config_mods::config::CONFIG;

use crate::mongo_integration::mongo_insert_data::mongo_insert_data;

use crate::postgres_integration::models::insertable::new_post::NewPost;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::write_error_posts_wrapper::write_error_posts_wrapper;
// for key in vec_of_provider_names.clone() {
//     let future_possible_drop_collection = mongo_drop_collection_wrapper(
//         mongo_url,
//         db_name_handle,
//         &format!("{}{}", key, db_collection_handle_second_part),
//         false,
//     );
//     match future_possible_drop_collection {
//         Ok(result_flag) => {
//             if result_flag {
//                 println!("drop done!");
//             } else {
//                 println!("drop fail with flag");
//             }
//         }
//         Err(e) => {
//             println!("drop fail with error {:#?}", e);
//         }
//     }
// }

#[deny(clippy::indexing_slicing)]
#[tokio::main]
pub async fn async_tokio_wrapper() {
    if CONFIG
        .params
        .enable_initialize_mongo_with_providers_link_parts
    {
        let vec_of_link_parts_hashmap = ProviderKind::get_providers_json_local_data_processed(
            ProviderKind::get_providers_json_local_data_unprocessed(),
        );
        if !vec_of_link_parts_hashmap.is_empty() {
            //todo: add check of doc already is in collection or add flag forse
            //todo add flag for provider
            let result_postgres_establish_connection =
                PgConnection::establish(&postgres_get_db_url());
            match result_postgres_establish_connection {
                Ok(pg_connection) => {
                    let _ = NewPost::insert_into_postgres(
                        &pg_connection,
                        NewPost {
                            title: "post_title",
                            body: "post_body",
                        },
                    );
                }
                Err(e) => {
                    print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        file!().to_string(),
                        line!().to_string(),
                        format!(
                            "PgConnection::establish {} error: {:#?}",
                            &postgres_get_db_url(),
                            e
                        ),
                    );
                }
            }
            let _ = mongo_insert_data(
                &CONFIG.mongo_params.providers_db_name_handle,
                vec_of_link_parts_hashmap,
            )
            .await;
        }
    }
    let option_tuple = check_new_posts_threads_parts().await;
    match option_tuple {
        Some((_posts, error_posts)) => {
            if !error_posts.is_empty() {
                let wrong_cases_thread = thread::spawn(move || {
                    block_on(write_error_posts_wrapper(error_posts));
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
