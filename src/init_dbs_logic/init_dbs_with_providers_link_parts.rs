use std::collections::HashMap;

use chrono::{DateTime, FixedOffset, Local, Utc};

use crate::config_mods::lazy_static_config::CONFIG;

use crate::helpers::where_was::WhereWas;
use crate::init_dbs_logic::init_mongo::init_mongo;
use crate::init_dbs_logic::init_mongo::InitMongoErrorEnum;
use crate::init_dbs_logic::init_postgres::init_postgres;
use crate::init_dbs_logic::init_postgres::PostgresInitErrorEnum;

use crate::postgres_integration::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length::PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum;
use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::providers_info::get_local_providers_link_parts::get_local_providers_link_parts;
use crate::providers::providers_info::get_local_providers_link_parts::GetLocalProvidersLinkPartsError;

use super::init_mongo::CollectionCountDocumentsOrIsNotEmpty;

use crate::postgres_integration::postgres_check_providers_link_parts_tables_are_empty::PostgresCheckProvidersLinkPartsTablesEmptyErrorEnum;
use crate::postgres_integration::postgres_create_providers_tables_if_not_exists::PostgresCreateProvidersDbsError;
use crate::postgres_integration::postgres_delete_all_from_providers_link_parts_tables::PostgresDeleteAllFromProvidersTablesError;
use crate::postgres_integration::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesError;

#[derive(Debug)]
pub enum InitDbsProvidersLinkPartsErrorEnum {
    GetLocalProvidersLinkParts {
        source: GetLocalProvidersLinkPartsError,
        where_was: WhereWas,
    },
    MongoClientOptionsParse {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    MongoClientWithOptions {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    MongoCollectionCountDocumentsOrIsNotEmpty {
        source: HashMap<ProviderKind, CollectionCountDocumentsOrIsNotEmpty>,
        where_was: WhereWas,
    },
    MongoInsertManyError {
        source: HashMap<ProviderKind, mongodb::error::Error>,
        where_was: WhereWas,
    },
    PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLength {
        source: PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum,
        where_was: WhereWas,
    },
    PostgresDeleteAllFromProvidersTables {
        source: PostgresDeleteAllFromProvidersTablesError,
        where_was: WhereWas,
    },
    PostgresCheckProvidersLinkPartsTablesEmptyError {
        source: PostgresCheckProvidersLinkPartsTablesEmptyErrorEnum,
        where_was: WhereWas,
    },
    PostgresCreateTableQueries {
        source: PostgresCreateProvidersDbsError,
        where_was: WhereWas,
    },
    PostgresInsertLinkPartsIntoProvidersTables {
        source: PostgresInsertLinkPartsIntoProvidersTablesError,
        where_was: WhereWas,
    },
    PostgresEstablishConnection {
        source: sqlx::Error,
        where_was: WhereWas,
    },
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn init_dbs_with_providers_link_parts(
) -> Result<(), Box<InitDbsProvidersLinkPartsErrorEnum>> {
    match get_local_providers_link_parts().await {
        Err(errors_hashmap) => Err(Box::new(
            InitDbsProvidersLinkPartsErrorEnum::GetLocalProvidersLinkParts {
                source: errors_hashmap,
                where_was: WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            },
        )),
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
                match *err {
                    InitMongoErrorEnum::ClientOptionsParse {
                        source,
                        where_was: _,
                    } => {
                        return Err(Box::new(
                            InitDbsProvidersLinkPartsErrorEnum::MongoClientOptionsParse {
                                source,
                                where_was: WhereWas {
                                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                    file: file!(),
                                    line: line!(),
                                    column: column!(),
                                },
                            },
                        ));
                    }
                    InitMongoErrorEnum::ClientWithOptions {
                        source,
                        where_was: _,
                    } => {
                        return Err(Box::new(
                            InitDbsProvidersLinkPartsErrorEnum::MongoClientWithOptions {
                                source,
                                where_was: WhereWas {
                                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                    file: file!(),
                                    line: line!(),
                                    column: column!(),
                                },
                            },
                        ));
                    }
                    InitMongoErrorEnum::CollectionCountDocumentsOrIsNotEmpty {
                        source,
                        where_was: _,
                    } => {
                        return Err( Box::new(InitDbsProvidersLinkPartsErrorEnum::MongoCollectionCountDocumentsOrIsNotEmpty {
                            source,
                            where_was: WhereWas {
                                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            },
                        }));
                    }
                    InitMongoErrorEnum::InsertManyError {
                        source,
                        where_was: _,
                    } => {
                        return Err(Box::new(
                            InitDbsProvidersLinkPartsErrorEnum::MongoInsertManyError {
                                source,
                                where_was: WhereWas {
                                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                    file: file!(),
                                    line: line!(),
                                    column: column!(),
                                },
                            },
                        ));
                    }
                }
            }
            if let Some(Err(err)) = postgres_insert_data_option_result {
                match *err {
                    PostgresInitErrorEnum::CheckProvidersLinksTablesLengthRowsEqualInitializationDataLength { source,where_was: _, } => {
                        return Err(Box::new(InitDbsProvidersLinkPartsErrorEnum::PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLength {
                            source,
                            where_was: WhereWas {
                                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc).with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            },
                        }));
                    }
                    PostgresInitErrorEnum::DeleteAllFromProvidersTables { source, where_was: _, } => {
                        return Err(Box::new(InitDbsProvidersLinkPartsErrorEnum::PostgresDeleteAllFromProvidersTables {
                            source,
                            where_was: WhereWas {
                                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc).with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            },
                        }));
                    }
                    PostgresInitErrorEnum::CheckProviderLinksTablesAreEmpty { source, where_was: _, } => {
                        return Err(Box::new(InitDbsProvidersLinkPartsErrorEnum::PostgresCheckProvidersLinkPartsTablesEmptyError {
                            source,
                            where_was: WhereWas {
                                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc).with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            },
                        }));
                    }
                    PostgresInitErrorEnum::CreateTableQueries { source, where_was: _, } => {
                        return Err(Box::new(InitDbsProvidersLinkPartsErrorEnum::PostgresCreateTableQueries {
                            source,
                            where_was: WhereWas {
                                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc).with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            },
                        }));
                    }
                    PostgresInitErrorEnum::InsertLinkPartsIntoProvidersTables { source, where_was: _, } => {
                        return Err(Box::new(InitDbsProvidersLinkPartsErrorEnum::PostgresInsertLinkPartsIntoProvidersTables {
                            source,
                            where_was: WhereWas {
                                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc).with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            },
                        }));
                    }
                    PostgresInitErrorEnum::EstablishConnection { source, where_was: _, } => {
                       return Err(Box::new(InitDbsProvidersLinkPartsErrorEnum::PostgresEstablishConnection {
                        source,
                        where_was: WhereWas {
                            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc).with_timezone(&FixedOffset::east(CONFIG.timezone)),
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    }));
                    }
                }
            }
            Ok(())
        }
    }
}
