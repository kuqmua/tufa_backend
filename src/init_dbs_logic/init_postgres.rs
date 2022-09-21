use std::collections::HashMap;
use std::time::Duration;
// use impl_get_where_was_for_error_struct::ImplGetWhereWasForErrorStruct;
use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use crate::postgres_integration::postgres_check_providers_link_parts_tables_are_empty::postgres_check_providers_link_parts_tables_are_empty;
use crate::postgres_integration::postgres_check_providers_link_parts_tables_are_empty::PostgresCheckProvidersLinkPartsTablesEmptyError;
use crate::postgres_integration::postgres_create_providers_tables_if_not_exists::postgres_create_providers_tables_if_not_exists;
use crate::postgres_integration::postgres_create_providers_tables_if_not_exists::PostgresCreateProvidersDbsError;
use crate::postgres_integration::postgres_delete_all_from_providers_link_parts_tables::postgres_delete_all_from_providers_link_parts_tables;
use crate::postgres_integration::postgres_delete_all_from_providers_link_parts_tables::PostgresDeleteAllFromProvidersTablesError;
use crate::postgres_integration::postgres_establish_connection::postgres_establish_connection;
use crate::postgres_integration::postgres_establish_connection::PostgresEstablishConnectionError;
use crate::postgres_integration::postgres_insert_link_parts_into_providers_tables::postgres_insert_link_parts_into_providers_tables;
use crate::postgres_integration::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesError;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use impl_get_source_for_parent_error_struct::ImplGetSourceForParentErrorStruct;
use impl_get_source_for_source_error_enum::ImplGetSourceForSourceErrorEnum;
use impl_get_where_was_one_or_many_for_enum::ImplGetWhereWasOneOrManyForEnum;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStruct;
use sqlx::postgres::PgPoolOptions;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::where_was::WhereWas;
use init_error_with_tracing_for_original_error_struct::InitErrorWithTracingForOriginalErrorStruct;
use crate::postgres_integration::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length::PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::with_tracing::WithTracing;
use init_error::InitError;
// use crate::postgres_integration::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length;

#[derive(
    Debug,
    InitError,
    ImplGetSourceForParentErrorStruct,
    ImplGetWhereWasOneOrManyOneForErrorStruct,
    InitErrorWithTracingForOriginalErrorStruct,
)]
pub struct PostgresInitError {
    source: PostgresInitErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetWhereWasOneOrManyForEnum, ImplGetSourceForSourceErrorEnum)]
pub enum PostgresInitErrorEnum {
    EstablishConnection(PostgresEstablishConnectionError),
    CreateTableQueries(PostgresCreateProvidersDbsError),
    CheckProviderLinksTablesAreEmpty(PostgresCheckProvidersLinkPartsTablesEmptyError),
    DeleteAllFromProvidersTables(PostgresDeleteAllFromProvidersTablesError),
    CheckProvidersLinksTablesLengthRowsEqualInitializationDataLength(
        PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError,
    ),
    InsertLinkPartsIntoProvidersTables(PostgresInsertLinkPartsIntoProvidersTablesError),
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn init_postgres(
    providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>,
    should_trace: bool,
) -> Result<(), Box<PostgresInitError>> {
    match postgres_establish_connection(&providers_json_local_data_hashmap, should_trace).await {
        Err(e) => Err(Box::new(PostgresInitError::init_error_with_possible_trace(
            PostgresInitErrorEnum::EstablishConnection(*e),
            WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            },
            &CONFIG.source_place_type,
            &GIT_INFO.data,
            should_trace,
        ))),
        Ok(pool) => {
            if let Err(e) = postgres_create_providers_tables_if_not_exists(
                &providers_json_local_data_hashmap,
                &pool,
                false,
            )
            .await
            {
                return Err(Box::new(PostgresInitError::init_error_with_possible_trace(
                    PostgresInitErrorEnum::CreateTableQueries(*e),
                    WhereWas {
                        time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                            .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                    &CONFIG.source_place_type,
                    &GIT_INFO.data,
                    should_trace,
                )));
            }
            if let Err(e) = postgres_check_providers_link_parts_tables_are_empty(
                &providers_json_local_data_hashmap,
                &pool,
                false,
            )
            .await
            {
                return Err(Box::new(PostgresInitError::init_error_with_possible_trace(
                    PostgresInitErrorEnum::CheckProviderLinksTablesAreEmpty(*e),
                    WhereWas {
                        time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                            .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                    &CONFIG.source_place_type,
                    &GIT_INFO.data,
                    should_trace,
                )));
            }
            if let Err(e) = postgres_delete_all_from_providers_link_parts_tables(
                &providers_json_local_data_hashmap,
                &pool,
                false,
            )
            .await
            {
                return Err(Box::new(PostgresInitError::init_error_with_possible_trace(
                    PostgresInitErrorEnum::DeleteAllFromProvidersTables(*e),
                    WhereWas {
                        time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                            .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                    &CONFIG.source_place_type,
                    &GIT_INFO.data,
                    should_trace,
                )));
            }
            // if let Err(e) = postgres_check_providers_links_tables_length_rows_equal_initialization_data_length(
            //     &providers_json_local_data_hashmap,
            //     &pool,
            //     false,
            // )
            // .await {
            //                                                                             return Err(Box::new(PostgresInitError::init_error_with_possible_trace(
            //     PostgresInitErrorEnum::CheckProvidersLinksTablesLengthRowsEqualInitializationDataLength(e),
            //     WhereWas {
            //         time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
            //             .with_timezone(&FixedOffset::east(CONFIG.timezone)),
            //         file: file!(),
            //         line: line!(),
            //         column: column!(),
            //     },
            //     &CONFIG.source_place_type,
            //     &GIT_INFO.data,
            //     should_trace,
            // )));
            // }
            if let Err(e) = postgres_insert_link_parts_into_providers_tables(
                &providers_json_local_data_hashmap,
                &pool,
                false,
            )
            .await
            {
                return Err(Box::new(PostgresInitError::init_error_with_possible_trace(
                    PostgresInitErrorEnum::InsertLinkPartsIntoProvidersTables(*e),
                    WhereWas {
                        time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                            .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                    &CONFIG.source_place_type,
                    &GIT_INFO.data,
                    should_trace,
                )));
            }
            Ok(())
        }
    }
}
