use mongodb::{options::ClientOptions, Client};

use chrono::{DateTime, FixedOffset, Local, Utc};

use crate::helpers::where_was::WhereWas;

use crate::config_mods::lazy_static_config::CONFIG;

#[derive(Debug)]
pub enum MongoDropEmptyDbErrorEnum {
    ClientOptionsParse {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    ClientWithOptions {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    ListCollectionNames {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    CollectionNamesListIsEmpty {
        source: String,
        where_was: WhereWas,
    },
    DatabaseDrop {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn mongo_drop_empty_db(
    mongo_url: &str,
    db_name: &str,
) -> Result<(), Box<MongoDropEmptyDbErrorEnum>> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => Err(Box::new(MongoDropEmptyDbErrorEnum::ClientOptionsParse {
            source: e,
            where_was: WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            },
        })),
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => Err(Box::new(MongoDropEmptyDbErrorEnum::ClientWithOptions {
                source: e,
                where_was: WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            })),
            Ok(client) => {
                let db = client.database(db_name);
                match db.list_collection_names(None).await {
                    Err(e) => Err(Box::new(MongoDropEmptyDbErrorEnum::ListCollectionNames {
                        source: e,
                        where_was: WhereWas {
                            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    })),
                    Ok(collections_names_list) => {
                        if !collections_names_list.is_empty() {
                            return Err(Box::new(
                                MongoDropEmptyDbErrorEnum::CollectionNamesListIsEmpty {
                                    source: db_name.to_string(),
                                    where_was: WhereWas {
                                        time: DateTime::<Utc>::from_utc(
                                            Local::now().naive_utc(),
                                            Utc,
                                        )
                                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                        file: file!(),
                                        line: line!(),
                                        column: column!(),
                                    },
                                },
                            ));
                        }
                        if let Err(e) = db.drop(None).await {
                            return Err(Box::new(MongoDropEmptyDbErrorEnum::DatabaseDrop {
                                source: e,
                                where_was: WhereWas {
                                    time: DateTime::<Utc>::from_utc(
                                        Local::now().naive_utc(),
                                        Utc,
                                    )
                                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                    file: file!(),
                                    line: line!(),
                                    column: column!(),
                                },
                            }));
                        }
                        Ok(())
                    }
                }
            }
        },
    }
}
