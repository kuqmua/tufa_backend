use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
// use crate::helpers::where_was::WhereWasOneOrFew;
use crate::traits::get_source::GetSource;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
// use crate::traits::get_where_was::GetWhereWas;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_get_source::ImplGetSource;
use impl_get_source_for_original_error_struct::ImplGetSourceForOriginalErrorStruct;
use impl_get_source_for_parent_error_struct::ImplGetSourceForParentErrorStruct;
use impl_get_source_for_simple_error_enum::ImplGetSourceForSimpleErrorEnum;
use impl_get_source_for_source_error_enum::ImplGetSourceForSourceErrorEnum;
use impl_get_where_was_for_error_struct::ImplGetWhereWasForErrorStruct;
use mongodb::options::ClientOptions;
use mongodb::Client;
use std::fmt;
use std::time::Duration;
// use init_error_with_tracing::DeriveInitErrorWithTracing;
//DeriveInitErrorWithTracing,
// use init_error::DeriveInitError;
// , DeriveInitError
#[derive(
    Debug,
    ImplGetSourceForParentErrorStruct,
    ImplGetWhereWasForErrorStruct,
    ImplDisplayForErrorStruct,
)]
pub struct MongoCheckAvailabilityError {
    source: MongoCheckAvailabilityErrorEnum,
    where_was: WhereWas,
}

impl MongoCheckAvailabilityError {
    pub fn with_tracing(source: MongoCheckAvailabilityErrorEnum, where_was: WhereWas) -> Self {
        match crate::config_mods::lazy_static_config::CONFIG.source_place_type {
            crate::config_mods::source_place_type::SourcePlaceType::Source => {
                tracing::error!(
                    error = format!("{}", source.get_source()),
                    source = where_was.source_place(),
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::Github => {
                tracing::error!(
                    error = format!("{}", source.get_source()),
                    github_source = where_was.github_source_place(),
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::None => {
                tracing::error!(error = format!("{}", source.get_source()));
            }
        }
        Self { source, where_was }
    }
    pub fn new(source: MongoCheckAvailabilityErrorEnum, where_was: WhereWas) -> Self {
        Self { source, where_was }
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
        Err(e) => {
            let where_was = WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            };
            match should_trace {
                true => Err(Box::new(MongoCheckAvailabilityError::with_tracing(
                    MongoCheckAvailabilityErrorEnum::ClientOptionsParse(e),
                    where_was,
                ))),
                false => Err(Box::new(MongoCheckAvailabilityError::new(
                    MongoCheckAvailabilityErrorEnum::ClientOptionsParse(e),
                    where_was,
                ))),
            }
        }
        Ok(mut client_options) => {
            client_options.connect_timeout =
                Some(Duration::from_millis(CONFIG.mongo_connection_timeout));
            match Client::with_options(client_options) {
                Err(e) => {
                    let where_was = WhereWas {
                        time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                            .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    };
                    match should_trace {
                        true => Err(Box::new(MongoCheckAvailabilityError::with_tracing(
                            MongoCheckAvailabilityErrorEnum::ClientWithOptions(e),
                            where_was,
                        ))),
                        false => Err(Box::new(MongoCheckAvailabilityError::new(
                            MongoCheckAvailabilityErrorEnum::ClientWithOptions(e),
                            where_was,
                        ))),
                    }
                }
                Ok(client) => {
                    if let Err(e) = client
                        .database(&CONFIG.mongo_providers_logs_db_name)
                        .list_collection_names(None)
                        .await
                    {
                        let where_was = WhereWas {
                            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        };
                        match should_trace {
                            true => {
                                return {
                                    Err(Box::new(MongoCheckAvailabilityError::with_tracing(
                                        MongoCheckAvailabilityErrorEnum::ListCollectionNames(e),
                                        where_was,
                                    )))
                                };
                            }
                            false => {
                                return Err(Box::new(MongoCheckAvailabilityError::new(
                                    MongoCheckAvailabilityErrorEnum::ListCollectionNames(e),
                                    where_was,
                                )));
                            }
                        }
                    }
                    Ok(())
                }
            }
        }
    }
}
