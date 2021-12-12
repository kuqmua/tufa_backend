use std::collections::HashMap;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::config_mods::lazy_static_config::CONFIG;

use crate::mongo_integration::mongo_insert_data::{mongo_insert_data, PutDataInMongoResult};

use crate::postgres_integration::models::insertable::queryable_link_part::InsertableLinkPart;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

use crate::providers::get_providers_json_local_data_processed_error::GetProvidersJsonLocalDataProcessedError;
use crate::providers::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::postgres_integration::models::queryable::queryable_link_part::QueryableLinkPart;
use crate::postgres_integration::schema::providers_link_parts::dsl::*;

#[derive(Debug)]
pub enum InitDbsError {
    GetProvidersJsonLocalData(HashMap<ProviderKind, GetProvidersJsonLocalDataProcessedError>),
    MongoInsertDataPartial, //todo: add values in here
    MongoInsertDataFailure,
    //todo: Postgres
}

#[deny(clippy::indexing_slicing)]
pub async fn init_dbs() -> Result<(), InitDbsError> {
    let providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>;
    let (success_hashmap, errors_hashmap) = ProviderKind::get_providers_json_local_data_processed();
    if !errors_hashmap.is_empty() {
        return Err(InitDbsError::GetProvidersJsonLocalData(errors_hashmap));
    }
    providers_json_local_data_hashmap = success_hashmap;
    let (mongo_insert_data_option_result, postgres_insert_data_option_result) = tokio::join!(
        async {
            if !CONFIG.mongo_enable_initialization {
                return None;
            }
            Some(
                mongo_insert_data(
                    &CONFIG.mongo_providers_logs_db_name,
                    providers_json_local_data_hashmap.clone(), //clone coz move in postgres part
                )
                .await,
            )
        },
        async {
            if !CONFIG.postgres_enable_initialization {
                return None;
            }
            let result_postgres_establish_connection =
                PgConnection::establish(&postgres_get_db_url());
            match result_postgres_establish_connection {
                Ok(pg_connection) => {
                    let results = providers_link_parts
                        .limit(5)
                        .load::<QueryableLinkPart>(&pg_connection)
                        .expect("Error loading providers_link_parts");

                    //
                    println!("work work {:#?}", results);
                    // let f = InsertableLinkPart::insert_into_postgres(
                    //     &pg_connection,
                    //     InsertableLinkPart {
                    //         link_part: "post_title",
                    //     },
                    // );
                    // match f {
                    //     Ok(_) => println!("ok"),
                    //     Err(e) => println!("rrr {:#?}", e),
                    // }
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
            Some(true)
        }
    );
    if let Some(result) = mongo_insert_data_option_result {
        match result {
            PutDataInMongoResult::Success => (),
            PutDataInMongoResult::PartialSuccess => {
                return Err(InitDbsError::MongoInsertDataPartial)
            }
            PutDataInMongoResult::Failure => return Err(InitDbsError::MongoInsertDataFailure),
        }
    }
    if let Some(_result) = postgres_insert_data_option_result {
        println!("todo postgres_insert_data_option_result");
    }
    Ok(())
}
