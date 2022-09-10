use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use crate::traits::get_source::GetSource;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_get_source_for_parent_error_struct::ImplGetSourceForParentErrorStruct;
use impl_get_source_for_simple_error_enum::ImplGetSourceForSimpleErrorEnum;
use impl_get_where_was_for_error_struct::ImplGetWhereWasForErrorStruct;
use init_error::InitError;
use init_error_with_tracing_for_original_error_struct::InitErrorWithTracingForOriginalErrorStruct;
use mongodb::options::ClientOptions;
use mongodb::Client;
use std::time::Duration;

#[derive(
    Debug,
    ImplGetSourceForParentErrorStruct,
    // ImplGetWhereWasForErrorStruct,
    ImplDisplayForErrorStruct,
    InitError,
    InitErrorWithTracingForOriginalErrorStruct,
)]
pub struct MongoCheckAvailabilityError {
    source: MongoCheckAvailabilityErrorEnum,
    where_was: WhereWas,
}

impl crate::traits::get_where_was_one_or_many::GetWhereWasOneOrMany
    for MongoCheckAvailabilityError
{
    fn get_where_was_one_or_many(&self) -> crate::helpers::where_was::WhereWasOneOrMany {
        crate::helpers::where_was::WhereWasOneOrMany::One(
            crate::helpers::where_was::WhereWasWithAddition {
                additional_info: None,
                where_was: self.where_was.clone(),
            },
        )
    }
}

impl MongoCheckAvailabilityError {
    fn init(
        source: MongoCheckAvailabilityErrorEnum,
        where_was: WhereWas,
        should_trace: bool,
    ) -> Self {
        match should_trace {
            true => Self::with_tracing(source, where_was),
            false => Self::new(source, where_was),
        }
    }
}

#[derive(Debug, ImplGetSourceForSimpleErrorEnum, ImplDisplayForSimpleErrorEnum)]
pub enum MongoCheckAvailabilityErrorEnum {
    ClientOptionsParse(mongodb::error::Error),
    ClientWithOptions(mongodb::error::Error),
    ListCollectionNames(mongodb::error::Error),
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn mongo_check_availability(
    mongo_url: &str,
    should_trace: bool,
) -> Result<(), Box<MongoCheckAvailabilityError>> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => Err(Box::new(MongoCheckAvailabilityError::init(
            MongoCheckAvailabilityErrorEnum::ClientOptionsParse(e),
            WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            },
            should_trace,
        ))),
        Ok(mut client_options) => {
            client_options.connect_timeout =
                Some(Duration::from_millis(CONFIG.mongo_connection_timeout));
            match Client::with_options(client_options) {
                Err(e) => Err(Box::new(MongoCheckAvailabilityError::init(
                    MongoCheckAvailabilityErrorEnum::ClientWithOptions(e),
                    WhereWas {
                        time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                            .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                    should_trace,
                ))),
                Ok(client) => {
                    if let Err(e) = client
                        .database(&CONFIG.mongo_providers_logs_db_name)
                        .list_collection_names(None)
                        .await
                    {
                        return Err(Box::new(MongoCheckAvailabilityError::init(
                            MongoCheckAvailabilityErrorEnum::ListCollectionNames(e),
                            WhereWas {
                                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            },
                            should_trace,
                        )));
                    }
                    Ok(())
                }
            }
        }
    }
}
