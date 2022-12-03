use crate::global_variables::runtime::config::CONFIG;
use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::postgres_integration::postgres_check_providers_link_parts_tables_are_empty::postgres_check_providers_link_parts_tables_are_empty;
use crate::postgres_integration::postgres_check_providers_link_parts_tables_are_empty::PostgresCheckProvidersLinkPartsTablesEmptyWrapperError;
use crate::postgres_integration::postgres_create_providers_tables_if_not_exists::postgres_create_providers_tables_if_not_exists;
use crate::postgres_integration::postgres_create_providers_tables_if_not_exists::PostgresCreateProvidersDbsOriginError;
use crate::postgres_integration::postgres_delete_all_from_providers_link_parts_tables::postgres_delete_all_from_providers_link_parts_tables;
use crate::postgres_integration::postgres_delete_all_from_providers_link_parts_tables::PostgresDeleteAllFromProvidersTablesOriginError;
use crate::postgres_integration::postgres_establish_connection::postgres_establish_connection;
use crate::postgres_integration::postgres_establish_connection::PostgresEstablishConnectionOriginError;
use crate::postgres_integration::postgres_insert_link_parts_into_providers_tables::postgres_insert_link_parts_into_providers_tables;
use crate::postgres_integration::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesOriginError;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use impl_get_source::ImplGetSourceFromTufaCommon;
use std::collections::HashMap;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromTufaCommon;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::common::where_was::WhereWas;
use impl_error_with_tracing::ImplErrorWithTracingFromTufaCommon;
use crate::postgres_integration::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length::PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperError;
use tufa_common::traits::get_source::GetSource;
use init_error::InitErrorFromTufaCommon;
use tufa_common::traits::get_log_with_additional_where_was::GetLogWithAdditionalWhereWas;
use impl_get_git_info::ImplGetGitInfoFromTufaCommon;
// use crate::postgres_integration::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length;

#[derive(
    Debug,
    InitErrorFromTufaCommon,
    ImplGetSourceFromTufaCommon,
    ImplGetWhereWasOriginOrWrapperFromTufaCommon,
    ImplErrorWithTracingFromTufaCommon,
    // ImplGetGitInfoFromTufaCommon,
)]
pub struct PostgresInitWrapperError {
    source: PostgresInitWrapperErrorEnum,
    where_was: WhereWas,
}

// impl PostgresInitWrapperError {
//     fn kekw(&self) -> String {
//         // use tufa_common::traits::get_git_info::GetGitInfo;
//         self.get_log_with_additional_where_wass()
//     }
// }
// pub trait GetLogWithAdditionalWhereWass<T> {
//     fn get_log_with_additional_where_wass(&self) -> String;
// }

// impl<T> GetLogWithAdditionalWhereWass<Self> for T
// where
//     Self: tufa_common::traits::get_git_info::GetGitInfo,
// {
//     fn get_log_with_additional_where_wass(&self) -> String {
//         String::from("kekw")
//     }
// }

// impl tufa_common::traits::get_git_info::GetGitInfo for PostgresInitWrapperError {
//     fn get_git_info(&self) -> &'static tufa_common::common::git::git_info::GitInformation<'static> {
//         &crate::global_variables::compile_time::git_info::GIT_INFO
//     }
// }

#[derive(Debug, ImplGetWhereWasOriginOrWrapperFromTufaCommon, ImplGetSourceFromTufaCommon)]
pub enum PostgresInitWrapperErrorEnum {
    EstablishConnectionWrapper(PostgresEstablishConnectionOriginError),
    CreateTableQueriesWrapper(PostgresCreateProvidersDbsOriginError),
    CheckProviderLinksTablesAreEmptyWrapper(PostgresCheckProvidersLinkPartsTablesEmptyWrapperError),
    DeleteAllFromProvidersTablesWrapper(PostgresDeleteAllFromProvidersTablesOriginError),
    CheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapper(
        PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperError,
    ),
    InsertLinkPartsIntoProvidersTablesWrapper(
        PostgresInsertLinkPartsIntoProvidersTablesOriginError,
    ),
}

pub async fn init_postgres(
    providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>,
    should_trace: bool,
) -> Result<(), Box<PostgresInitWrapperError>> {
    match postgres_establish_connection(&providers_json_local_data_hashmap, should_trace).await {
        Err(e) => Err(Box::new(
            PostgresInitWrapperError::init_error_with_possible_trace(
                PostgresInitWrapperErrorEnum::EstablishConnectionWrapper(*e),
                WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    file: String::from(file!()),
                    line: line!(),
                    column: column!(),
                    git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                },
                &CONFIG.source_place_type,
                should_trace,
            ),
        )),
        Ok(pool) => {
            if let Err(e) = postgres_create_providers_tables_if_not_exists(
                &providers_json_local_data_hashmap,
                &pool,
                false,
            )
            .await
            {
                return Err(Box::new(
                    PostgresInitWrapperError::init_error_with_possible_trace(
                        PostgresInitWrapperErrorEnum::CreateTableQueriesWrapper(*e),
                        WhereWas {
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                            file: String::from(file!()),
                            line: line!(),
                            column: column!(),
                            git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                        },
                        &CONFIG.source_place_type,
                        should_trace,
                    ),
                ));
            }
            if let Err(e) = postgres_check_providers_link_parts_tables_are_empty(
                &providers_json_local_data_hashmap,
                &pool,
                false,
            )
            .await
            {
                return Err(Box::new(
                    PostgresInitWrapperError::init_error_with_possible_trace(
                        PostgresInitWrapperErrorEnum::CheckProviderLinksTablesAreEmptyWrapper(*e),
                        WhereWas {
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                            file: String::from(file!()),
                            line: line!(),
                            column: column!(),
                            git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                        },
                        &CONFIG.source_place_type,
                        should_trace,
                    ),
                ));
            }
            if let Err(e) = postgres_delete_all_from_providers_link_parts_tables(
                &providers_json_local_data_hashmap,
                &pool,
                false,
            )
            .await
            {
                return Err(Box::new(
                    PostgresInitWrapperError::init_error_with_possible_trace(
                        PostgresInitWrapperErrorEnum::DeleteAllFromProvidersTablesWrapper(*e),
                        WhereWas {
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                            file: String::from(file!()),
                            line: line!(),
                            column: column!(),
                            git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                        },
                        &CONFIG.source_place_type,
                        should_trace,
                    ),
                ));
            }
            // if let Err(e) = postgres_check_providers_links_tables_length_rows_equal_initialization_data_length(
            //     &providers_json_local_data_hashmap,
            //     &pool,
            //     false,
            // )
            // .await {
            //                                                                             return Err(Box::new(PostgresInitWrapperError::init_error_with_possible_trace(
            //     PostgresInitWrapperErrorEnum::CheckProvidersLinksTablesLengthRowsEqualInitializationDataLength(e),
            //     WhereWas {
            //         time: std::time::SystemTime::now()
            // .duration_since(std::time::UNIX_EPOCH)
            // .expect("cannot convert time to unix_epoch"),
            //         file: file!(),
            //         line: line!(),
            //         column: column!(),
            //     },
            //     &CONFIG.source_place_type,
            //     &GIT_INFO,
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
                return Err(Box::new(
                    PostgresInitWrapperError::init_error_with_possible_trace(
                        PostgresInitWrapperErrorEnum::InsertLinkPartsIntoProvidersTablesWrapper(*e),
                        WhereWas {
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                            file: String::from(file!()),
                            line: line!(),
                            column: column!(),
                            git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                        },
                        &CONFIG.source_place_type,
                        should_trace,
                    ),
                ));
            }
            Ok(())
        }
    }
}
