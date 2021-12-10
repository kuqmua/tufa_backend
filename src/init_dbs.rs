use std::collections::HashMap;
// use std::thread;

// use diesel::pg::PgConnection;
// use diesel::prelude::*;
// use futures::executor::block_on;

// use crate::helpers::resource::{Resource, ResourceError};

use crate::config_mods::lazy_static_config::CONFIG;

use crate::mongo_integration::mongo_insert_data::{mongo_insert_data, PutDataInMongoResult};

// use crate::postgres_integration::models::insertable::new_post::NewPost;
// use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

use crate::providers::get_providers_json_local_data_processed_error::GetProvidersJsonLocalDataProcessedError;
use crate::providers::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;

#[derive(Debug)]
pub enum InitDbsError {
    GetProvidersJsonLocalData(HashMap<ProviderKind, GetProvidersJsonLocalDataProcessedError>),
    GetProvidersJsonLocalDataSuccessIsEmpty,
    MongoInsertDataPartial, //todo: add values in here
    MongoInsertDataFailure,
    //todo: Postgres
}

#[deny(clippy::indexing_slicing)]
pub async fn init_dbs() -> Result<(), InitDbsError> {
    let providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>;
    let (success_hashmap, errors_hashmap) = ProviderKind::get_providers_json_local_data_processed();
    if !errors_hashmap.is_empty() {
        //todo: maybe add some logic for partial success?
        return Err(InitDbsError::GetProvidersJsonLocalData(errors_hashmap));
    }
    if !success_hashmap.is_empty() {
        return Err(InitDbsError::GetProvidersJsonLocalDataSuccessIsEmpty);
    }
    providers_json_local_data_hashmap = success_hashmap;
    //todo: make it parrallel
    if CONFIG.mongo_enable_initialization {
        match mongo_insert_data(
            &CONFIG.mongo_providers_logs_db_name,
            providers_json_local_data_hashmap,
        )
        .await
        {
            PutDataInMongoResult::Success => (),
            PutDataInMongoResult::PartialSuccess => {
                return Err(InitDbsError::MongoInsertDataPartial)
            }
            PutDataInMongoResult::Failure => return Err(InitDbsError::MongoInsertDataFailure),
        }
    }
    //todo: make it parrallel
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
    Ok(())
}
