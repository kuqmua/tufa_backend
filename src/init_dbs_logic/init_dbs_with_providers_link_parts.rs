use std::collections::HashMap;

use crate::config_mods::lazy_static_config::CONFIG;

use crate::init_dbs_logic::init_mongo::init_mongo;
use crate::init_dbs_logic::init_mongo::InitMongoErrorEnum;
use crate::init_dbs_logic::init_postgres::init_postgres;
use crate::init_dbs_logic::init_postgres::PostgresInitErrorEnum;

use crate::postgres_integration::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length::PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError;
use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::providers_info::get_local_providers_link_parts::get_local_providers_link_parts;
use crate::providers::providers_info::get_local_providers_link_parts::GetLocalProvidersLinkPartsError;

use super::init_mongo::CollectionCountDocumentsOrIsNotEmpty;

use crate::postgres_integration::postgres_check_providers_link_parts_tables_are_empty::PostgresCheckProvidersLinkPartsTablesEmptyError;
use crate::postgres_integration::postgres_create_providers_tables_if_not_exists::PostgresCreateProvidersDbsError;
use crate::postgres_integration::postgres_delete_all_from_providers_link_parts_tables::PostgresDeleteAllFromProvidersTablesError;
use crate::postgres_integration::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesError;

#[derive(Debug)]
pub struct InitDbsProvidersLinkPartsError {
    pub source: Box<InitDbsProvidersLinkPartsErrorEnum>,
}

#[derive(Debug)]
pub enum InitDbsProvidersLinkPartsErrorEnum {
    GetLocalProvidersLinkParts {
        source: GetLocalProvidersLinkPartsError,
        file: &'static str,
        line: u32,
        column: u32,
    },
    MongoClientOptionsParse {
        source: mongodb::error::Error,
        file: &'static str,
        line: u32,
        column: u32,
    },
    MongoClientWithOptions {
        source: mongodb::error::Error,
        file: &'static str,
        line: u32,
        column: u32,
    },
    MongoCollectionCountDocumentsOrIsNotEmpty {
        source: HashMap<ProviderKind, CollectionCountDocumentsOrIsNotEmpty>,

        file: &'static str,
        line: u32,
        column: u32,
    },
    MongoInsertManyError {
        source: HashMap<ProviderKind, mongodb::error::Error>,
        file: &'static str,
        line: u32,
        column: u32,
    },
    PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLength {
        source: PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError,

        file: &'static str,
        line: u32,
        column: u32,
    },
    PostgresDeleteAllFromProvidersTables {
        source: PostgresDeleteAllFromProvidersTablesError,
        file: &'static str,
        line: u32,
        column: u32,
    },
    PostgresCheckProvidersLinkPartsTablesEmptyError {
        source: PostgresCheckProvidersLinkPartsTablesEmptyError,

        file: &'static str,
        line: u32,
        column: u32,
    },
    PostgresCreateTableQueries {
        source: PostgresCreateProvidersDbsError,
        file: &'static str,
        line: u32,
        column: u32,
    },
    PostgresInsertLinkPartsIntoProvidersTables {
        source: PostgresInsertLinkPartsIntoProvidersTablesError,
        file: &'static str,
        line: u32,
        column: u32,
    },
    PostgresEstablishConnection {
        source: sqlx::Error,
        file: &'static str,
        line: u32,
        column: u32,
    },
}

#[deny(clippy::indexing_slicing)]
pub async fn init_dbs_with_providers_link_parts() -> Result<(), InitDbsProvidersLinkPartsError> {
    match get_local_providers_link_parts().await {
        Err(errors_hashmap) => Err(InitDbsProvidersLinkPartsError {
            source: Box::new(
                InitDbsProvidersLinkPartsErrorEnum::GetLocalProvidersLinkParts {
                    source: errors_hashmap,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            ),
        }),
        Ok(providers_json_local_data_hashmap) => {
            let providers_json_local_data_hashmap_clone = providers_json_local_data_hashmap.clone();
            let (mongo_insert_data_option_result, postgres_insert_data_option_result) = tokio::join!(
                async {
                    if CONFIG.is_mongo_initialization_enabled {
                        return Some(init_mongo(providers_json_local_data_hashmap).await);
                    }
                    None
                },
                async {
                    if CONFIG.is_postgres_initialization_enabled {
                        return Some(init_postgres(providers_json_local_data_hashmap_clone).await);
                    }
                    None
                }
            );
            if let Some(Err(err)) = mongo_insert_data_option_result {
                match *err.source {
                    InitMongoErrorEnum::ClientOptionsParse {
                        source,
                        file,
                        line,
                        column,
                    } => {
                        return Err(InitDbsProvidersLinkPartsError {
                            source: Box::new(
                                InitDbsProvidersLinkPartsErrorEnum::MongoClientOptionsParse {
                                    source,
                                    file: file!(),
                                    line: line!(),
                                    column: column!(),
                                },
                            ),
                        });
                    }
                    InitMongoErrorEnum::ClientWithOptions {
                        source,
                        file,
                        line,
                        column,
                    } => {
                        return Err(InitDbsProvidersLinkPartsError {
                            source: Box::new(
                                InitDbsProvidersLinkPartsErrorEnum::MongoClientWithOptions {
                                    source,
                                    file: file!(),
                                    line: line!(),
                                    column: column!(),
                                },
                            ),
                        });
                    }
                    InitMongoErrorEnum::CollectionCountDocumentsOrIsNotEmpty {
                        source,
                        file,
                        line,
                        column,
                    } => {
                        return Err(InitDbsProvidersLinkPartsError{
                            source: Box::new(InitDbsProvidersLinkPartsErrorEnum::MongoCollectionCountDocumentsOrIsNotEmpty {
                                source,
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            }),
                        });
                    }
                    InitMongoErrorEnum::InsertManyError {
                        source,
                        file,
                        line,
                        column,
                    } => {
                        return Err(InitDbsProvidersLinkPartsError {
                            source: Box::new(
                                InitDbsProvidersLinkPartsErrorEnum::MongoInsertManyError {
                                    source,
                                    file: file!(),
                                    line: line!(),
                                    column: column!(),
                                },
                            ),
                        });
                    }
                }
            }
            if let Some(Err(err)) = postgres_insert_data_option_result {
                match *err.source {
                    PostgresInitErrorEnum::CheckProvidersLinksTablesLengthRowsEqualInitializationDataLength { source, file, line, column } => {
                        return Err(InitDbsProvidersLinkPartsError{
                            source: Box::new(InitDbsProvidersLinkPartsErrorEnum::PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLength {
                                source,
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            }),
                        });
                    }
                    PostgresInitErrorEnum::DeleteAllFromProvidersTables { source, file, line, column } => {
                        return Err(InitDbsProvidersLinkPartsError{
                            source: Box::new(InitDbsProvidersLinkPartsErrorEnum::PostgresDeleteAllFromProvidersTables {
                                source,
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            }),
                        });
                    }
                    PostgresInitErrorEnum::CheckProviderLinksTablesAreEmpty { source, file, line, column } => {
                        return Err(InitDbsProvidersLinkPartsError{
                            source: Box::new(InitDbsProvidersLinkPartsErrorEnum::PostgresCheckProvidersLinkPartsTablesEmptyError {
                                source,
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            }),
                        });
                    }
                    PostgresInitErrorEnum::CreateTableQueries { source, file, line, column } => {
                        return Err(InitDbsProvidersLinkPartsError{
                            source: Box::new(InitDbsProvidersLinkPartsErrorEnum::PostgresCreateTableQueries {
                                source,
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            }),
                        });
                    }
                    PostgresInitErrorEnum::InsertLinkPartsIntoProvidersTables { source, file, line, column } => {
                        return Err(InitDbsProvidersLinkPartsError{
                            source: Box::new(InitDbsProvidersLinkPartsErrorEnum::PostgresInsertLinkPartsIntoProvidersTables {
                                source,
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            }),
                        });
                    }
                    PostgresInitErrorEnum::EstablishConnection { source, file, line, column } => {
                       return Err(InitDbsProvidersLinkPartsError{
                            source: Box::new(InitDbsProvidersLinkPartsErrorEnum::PostgresEstablishConnection {
                                source,
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            }),
                        });
                    }
                }
            }
            Ok(())
        }
    }
}
