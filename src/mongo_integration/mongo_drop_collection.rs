use mongodb::bson::Document;
use mongodb::Collection;
use mongodb::{options::ClientOptions, Client};

use chrono::{DateTime, FixedOffset, Local, Utc};

use crate::helpers::where_was::WhereWas;

use crate::config_mods::lazy_static_config::CONFIG;

#[derive(Debug)]
pub enum MongoDropCollectionErrorEnum {
    ClientOptionsParse {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    ClientWithOptions {
        source: mongodb::error::Error,
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
pub async fn mongo_drop_collection(
    mongo_url: &str,
    db_name: &str,
    db_collection_name: &str,
) -> Result<(), Box<MongoDropCollectionErrorEnum>> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => Err(Box::new(MongoDropCollectionErrorEnum::ClientOptionsParse {
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
            Err(e) => Err(Box::new(MongoDropCollectionErrorEnum::ClientWithOptions {
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
                let collection: Collection<Document> =
                    client.database(db_name).collection(db_collection_name);
                if let Err(e) = collection.drop(None).await {
                    return Err(Box::new(MongoDropCollectionErrorEnum::DatabaseDrop {
                        source: e,
                        where_was: WhereWas {
                            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    }));
                }
                Ok(())
            }
        },
    }
}
