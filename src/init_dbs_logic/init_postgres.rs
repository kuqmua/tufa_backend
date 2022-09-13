use std::collections::HashMap;
use std::time::Duration;
// use impl_get_where_was_for_error_struct::ImplGetWhereWasForErrorStruct;
use sqlx::postgres::PgPoolOptions;
use chrono::Utc;
use chrono::Local;
use chrono::FixedOffset;
use chrono::DateTime;
use crate::config_mods::lazy_static_config::CONFIG;
use tufa_common::where_was::WhereWas;
use crate::postgres_integration::postgres_delete_all_from_providers_link_parts_tables::postgres_delete_all_from_providers_link_parts_tables;
use crate::postgres_integration::postgres_delete_all_from_providers_link_parts_tables::PostgresDeleteAllFromProvidersTablesError;
use crate::postgres_integration::postgres_insert_link_parts_into_providers_tables::postgres_insert_link_parts_into_providers_tables;
use crate::postgres_integration::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesError;
use crate::postgres_integration::postgres_establish_connection::PostgresEstablishConnectionError;
use crate::postgres_integration::postgres_establish_connection::postgres_establish_connection;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::postgres_integration::postgres_check_providers_link_parts_tables_are_empty::postgres_check_providers_link_parts_tables_are_empty;
use crate::postgres_integration::postgres_check_providers_link_parts_tables_are_empty::PostgresCheckProvidersLinkPartsTablesEmptyError;
use crate::postgres_integration::postgres_create_providers_tables_if_not_exists::postgres_create_providers_tables_if_not_exists;
use crate::postgres_integration::postgres_create_providers_tables_if_not_exists::PostgresCreateProvidersDbsError;
use crate::helpers::postgres::get_postgres_url::get_postgres_url;
use crate::postgres_integration::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length::PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::with_tracing::WithTracing;
use init_error::InitError;
// use crate::postgres_integration::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length;

#[derive(Debug, InitError)] //, ImplGetWhereWasForErrorStruct
pub struct PostgresInitError {
    source: PostgresInitErrorEnum,
    where_was: WhereWas,
}

impl tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany for PostgresInitError {
    fn get_where_was_one_or_many(&self) -> tufa_common::where_was::WhereWasOneOrMany {
        tufa_common::where_was::WhereWasOneOrMany::One(
            tufa_common::where_was::WhereWasWithAddition {
                additional_info: None,
                where_was: self.where_was.clone(),
            },
        )
    }
}
// impl crate::traits::get_where_was_one_or_many::GetWhereWas for PostgresInitError {
//     fn get_where_was(&self) -> String {
//         match crate::config_mods::lazy_static_config::CONFIG.is_debug_implementation_enable {
//             true => format!("{:#?} {:#?}", self.where_was, self.source.get_where_was()),
//             false => format!("{} {}", self.where_was, self.source.get_where_was()),
//         }
//     }
// }

#[derive(Debug)]
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

impl tufa_common::traits::get_source::GetSource for PostgresInitErrorEnum {
    fn get_source(&self) -> String {
        match crate::config_mods::lazy_static_config::CONFIG.is_debug_implementation_enable {
            true => format!("{:#?}", self),
            false => match self {
                PostgresInitErrorEnum::EstablishConnection(e) => e.get_source(),
                PostgresInitErrorEnum::CreateTableQueries(e) => e.get_source(),
                PostgresInitErrorEnum::CheckProviderLinksTablesAreEmpty(e) => e.get_source(),
                PostgresInitErrorEnum::DeleteAllFromProvidersTables(e) => e.get_source(),
                PostgresInitErrorEnum::CheckProvidersLinksTablesLengthRowsEqualInitializationDataLength(e) => e.get_source(),
                PostgresInitErrorEnum::InsertLinkPartsIntoProvidersTables(e) => e.get_source(),
            },
        }
    }
}

impl tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany
    for PostgresInitErrorEnum
{
    fn get_where_was_one_or_many(&self) -> tufa_common::where_was::WhereWasOneOrMany {
        todo!()
        //         match self {
        //     PostgresInitErrorEnum::EstablishConnection(_e) => String::from(""),
        //     PostgresInitErrorEnum::CreateTableQueries(e) => e.get_where_was(),
        //     PostgresInitErrorEnum::CheckProviderLinksTablesAreEmpty(e) => e.get_where_was(),
        //     PostgresInitErrorEnum::DeleteAllFromProvidersTables(e) => e.get_where_was(),
        //     PostgresInitErrorEnum::CheckProvidersLinksTablesLengthRowsEqualInitializationDataLength(e) => e.get_where_was(),
        //     PostgresInitErrorEnum::InsertLinkPartsIntoProvidersTables(e) => e.get_where_was(),
        // }
        // tufa_common::where_was::WhereWasOneOrMany::One(self.where_was.clone())
    }
}

// impl crate::traits::get_where_was_one_or_many::GetWhereWas for PostgresInitErrorEnum {
//     fn get_where_was(&self) -> String {
//         match self {
//             PostgresInitErrorEnum::EstablishConnection(_e) => String::from(""),
//             PostgresInitErrorEnum::CreateTableQueries(e) => e.get_where_was(),
//             PostgresInitErrorEnum::CheckProviderLinksTablesAreEmpty(e) => e.get_where_was(),
//             PostgresInitErrorEnum::DeleteAllFromProvidersTables(e) => e.get_where_was(),
//             PostgresInitErrorEnum::CheckProvidersLinksTablesLengthRowsEqualInitializationDataLength(e) => e.get_where_was(),
//             PostgresInitErrorEnum::InsertLinkPartsIntoProvidersTables(e) => e.get_where_was(),
//         }
//     }
// }

impl tufa_common::traits::with_tracing::WithTracing<PostgresInitErrorEnum> for PostgresInitError {
    fn with_tracing(source: PostgresInitErrorEnum, where_was: WhereWas) -> Self {
        match crate::config_mods::lazy_static_config::CONFIG.source_place_type {
            crate::config_mods::source_place_type::SourcePlaceType::Source => {
                tracing::error!(
                    error = source.get_source(),
                    source_place = where_was.file_line_column(),
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::Github => {
                tracing::error!(
                    error = source.get_source(),
                    github_source_place =
                        where_was.github_file_line_column(&crate::helpers::git_info::GIT_INFO.data),
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::None => {
                tracing::error!(error = source.get_source());
            }
        }
        Self { source, where_was }
    }
}

impl tufa_common::traits::get_source::GetSource for PostgresInitError {
    fn get_source(&self) -> String {
        match crate::config_mods::lazy_static_config::CONFIG.is_debug_implementation_enable {
            true => format!("{:#?}", self.source),
            false => self.get_source(),
        }
    }
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
        Err(e) => {
            let where_was = WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            };
            match should_trace {
                true => Err(Box::new(PostgresInitError::with_tracing(
                    PostgresInitErrorEnum::EstablishConnection(*e),
                    where_was,
                ))),
                false => Err(Box::new(PostgresInitError::new(
                    PostgresInitErrorEnum::EstablishConnection(*e),
                    where_was,
                ))),
            }
        }
        Ok(pool) => {
            if let Err(e) = postgres_create_providers_tables_if_not_exists(
                &providers_json_local_data_hashmap,
                &pool,
                false,
            )
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
                        return Err(Box::new(PostgresInitError::with_tracing(
                            PostgresInitErrorEnum::CreateTableQueries(*e),
                            where_was,
                        )));
                    }
                    false => {
                        return Err(Box::new(PostgresInitError::new(
                            PostgresInitErrorEnum::CreateTableQueries(*e),
                            where_was,
                        )));
                    }
                }
            }
            if let Err(e) = postgres_check_providers_link_parts_tables_are_empty(
                &providers_json_local_data_hashmap,
                &pool,
                false,
            )
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
                        return Err(Box::new(PostgresInitError::with_tracing(
                            PostgresInitErrorEnum::CheckProviderLinksTablesAreEmpty(*e),
                            where_was,
                        )));
                    }
                    false => {
                        return Err(Box::new(PostgresInitError::new(
                            PostgresInitErrorEnum::CheckProviderLinksTablesAreEmpty(*e),
                            where_was,
                        )));
                    }
                }
            }
            if let Err(e) = postgres_delete_all_from_providers_link_parts_tables(
                &providers_json_local_data_hashmap,
                &pool,
                false,
            )
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
                        return Err(Box::new(PostgresInitError::with_tracing(
                            PostgresInitErrorEnum::DeleteAllFromProvidersTables(*e),
                            where_was,
                        )));
                    }
                    false => {
                        return Err(Box::new(PostgresInitError::new(
                            PostgresInitErrorEnum::DeleteAllFromProvidersTables(*e),
                            where_was,
                        )));
                    }
                }
            }
            // if let Err(e) = postgres_check_providers_links_tables_length_rows_equal_initialization_data_length(
            //     &providers_json_local_data_hashmap,
            //     &pool,
            //     false,
            // )
            // .await {
            //     let where_was = WhereWas {
            //         time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
            //             .with_timezone(&FixedOffset::east(CONFIG.timezone)),
            //         file: file!(),
            //         line: line!(),
            //         column: column!(),
            //     };
            //     match should_trace {
            //         true => {
            //             return Err(Box::new(PostgresInitError::with_tracing(PostgresInitErrorEnum::CheckProvidersLinksTablesLengthRowsEqualInitializationDataLength(e), where_was)));
            //         }
            //         false => {
            //             return Err(Box::new(PostgresInitError::new(PostgresInitErrorEnum::CheckProvidersLinksTablesLengthRowsEqualInitializationDataLength(e), where_was)));
            //         }
            //     }
            // }
            if let Err(e) = postgres_insert_link_parts_into_providers_tables(
                &providers_json_local_data_hashmap,
                &pool,
                false,
            )
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
                        return Err(Box::new(PostgresInitError::with_tracing(
                            PostgresInitErrorEnum::InsertLinkPartsIntoProvidersTables(*e),
                            where_was,
                        )));
                    }
                    false => {
                        return Err(Box::new(PostgresInitError::new(
                            PostgresInitErrorEnum::InsertLinkPartsIntoProvidersTables(*e),
                            where_was,
                        )));
                    }
                }
            }
            Ok(())
        }
    }
}
