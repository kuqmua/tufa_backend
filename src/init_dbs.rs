use std::collections::HashMap;
// use std::thread;

// use diesel::pg::PgConnection;
// use diesel::prelude::*;
// use futures::executor::block_on;

// use crate::helpers::resource::{Resource, ResourceError};
use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::config_mods::lazy_static_config::CONFIG;

use crate::mongo_integration::mongo_insert_data::mongo_insert_data;

// use crate::postgres_integration::models::insertable::new_post::NewPost;
// use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

use crate::providers::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;

#[deny(clippy::indexing_slicing)]
pub async fn init_dbs() {
    let providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>;
    if !ProviderKind::get_enabled_providers_vec().is_empty() {
        let (success_hashmap, errors_hashmap) =
            ProviderKind::get_providers_json_local_data_processed();
        if !errors_hashmap.is_empty() {
            print_colorful_message(
                None,
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                format!("providers_json_local_data error {:#?}", errors_hashmap),
            );
            return;
        }
        if !success_hashmap.is_empty() {
            print_colorful_message(
                None,
                PrintType::Info,
                file!().to_string(),
                line!().to_string(),
                format!("providers_json_local_data success_hashmap is empty"),
            );
            return;
        }
        providers_json_local_data_hashmap = success_hashmap;
    } else {
        print_colorful_message(
            None,
            PrintType::Info,
            file!().to_string(),
            line!().to_string(),
            "providers initialization disabled".to_owned(),
        );
        return; //remove later
    }

    if CONFIG.mongo_enable_initialization {
        let result = mongo_insert_data(
            &CONFIG.mongo_providers_logs_db_name,
            providers_json_local_data_hashmap,
        )
        .await;
    }
    if CONFIG.postgres_enable_initialization {
        //todo
        // let result_postgres_establish_connection = PgConnection::establish(&postgres_get_db_url());
        // match result_postgres_establish_connection {
        //     Ok(pg_connection) => {
        //         let _ = NewPost::insert_into_postgres(
        //             &pg_connection,
        //             NewPost {
        //                 title: "post_title",
        //                 body: "post_body",
        //             },
        //         );
        //     }
        //     Err(e) => {
        //         print_colorful_message(
        //             None,
        //             PrintType::WarningHigh,
        //             file!().to_string(),
        //             line!().to_string(),
        //             format!(
        //                 "PgConnection::establish {} error: {:#?}",
        //                 &postgres_get_db_url(),
        //                 e
        //             ),
        //         );
        //     }
        // }
    }
}
