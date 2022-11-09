use crate::once_cell_globals::config::CONFIG;
use crate::once_cell_globals::git_info::GIT_INFO;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_error_with_tracing_for_struct_with_get_source_without_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWasFromTufaCommon;
use impl_get_source_with_method::ImplGetSourceWithMethodFromTufaCommon;
use impl_get_source_without_method::ImplGetSourceWithoutMethodFromTufaCommon;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use mongodb::error::Error;
use mongodb::options::ClientOptions;
use mongodb::Client;
use std::time::Duration;
use tufa_common::common::where_was::WhereWas;
use tufa_common::common::where_was::WhereWasOneOrMany;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_trait::WhereWasTrait;

#[derive(
    Debug,
    ImplGetSourceWithMethodFromTufaCommon,
    ImplDisplayForErrorStruct,
    InitErrorFromTufaCommon,
    ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWasFromTufaCommon,
    ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon,
)]
pub struct MongoCheckAvailabilityError {
    source: MongoCheckAvailabilityErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceWithoutMethodFromTufaCommon, ImplDisplayForSimpleErrorEnum)]
pub enum MongoCheckAvailabilityErrorEnum {
    ClientOptionsParse(Error),
    ClientWithOptions(Error),
    ListCollectionNames(Error),
}

// //
// #[derive(
//     Debug,
//     ImplGetSourceWithMethodFromTufaCommon,
//     ImplDisplayForErrorStruct,
//     InitErrorFromTufaCommon,
//     ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWasFromTufaCommon,
//     ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon,
// )]
// pub struct MongoCheckAvailabilityError {
//     source: mongodb::error::Error,
//     where_was: WhereWas,
// }
// //
// impl From<mongodb::error::Error> for MongoCheckAvailabilityError {
//     fn from(e: mongodb::error::Error) -> Self {
//         let location = location();
//         Self {
//             source: mongodb::error::Error,
//             where_was: WhereWas {
//                 time: std::time::SystemTime::now()
// .duration_since(std::time::UNIX_EPOCH)
// .expect("cannot convert time to unix_epoch"),
//                 location: *core::panic::Location::caller(),
//             },
//         }
//     }
// }

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
            return Err(Box::new(
                MongoCheckAvailabilityError::init_error_with_possible_trace(
                    MongoCheckAvailabilityErrorEnum::ClientOptionsParse(e),
                    WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                    &CONFIG.source_place_type,
                    &GIT_INFO,
                    should_trace,
                ),
            ));
        }
        Ok(mut client_options) => {
            client_options.connect_timeout =
                Some(Duration::from_millis(CONFIG.mongo_connection_timeout));
            match Client::with_options(client_options) {
                Err(e) => Err(Box::new(
                    MongoCheckAvailabilityError::init_error_with_possible_trace(
                        MongoCheckAvailabilityErrorEnum::ClientWithOptions(e),
                        WhereWas {
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                            location: *core::panic::Location::caller(),
                        },
                        &CONFIG.source_place_type,
                        &GIT_INFO,
                        should_trace,
                    ),
                )),
                Ok(client) => {
                    if let Err(e) = client
                        .database(&CONFIG.mongo_providers_logs_db_name)
                        .list_collection_names(None)
                        .await
                    {
                        return Err(Box::new(
                            MongoCheckAvailabilityError::init_error_with_possible_trace(
                                MongoCheckAvailabilityErrorEnum::ListCollectionNames(e),
                                WhereWas {
                                    time: std::time::SystemTime::now()
                                        .duration_since(std::time::UNIX_EPOCH)
                                        .expect("cannot convert time to unix_epoch"),
                                    location: *core::panic::Location::caller(),
                                },
                                &CONFIG.source_place_type,
                                &GIT_INFO,
                                should_trace,
                            ),
                        ));
                    }
                    Ok(())
                }
            }
        }
    }
}
