use mongodb::options::ClientOptions;
use mongodb::Client;
use tufa_common::common::where_was::WhereWas;

#[derive(Debug)]
pub enum MongoDropDbErrorEnum {
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
pub async fn mongo_drop_db(
    mongo_url: &str,
    db_name: &str,
) -> Result<(), Box<MongoDropDbErrorEnum>> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => Err(Box::new(MongoDropDbErrorEnum::ClientOptionsParse {
            source: e,
            where_was: WhereWas {
                time: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("cannot convert time to unix_epoch"),
                location: *core::panic::Location::caller(),
            },
        })),
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => Err(Box::new(MongoDropDbErrorEnum::ClientWithOptions {
                source: e,
                where_was: WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    location: *core::panic::Location::caller(),
                },
            })),
            Ok(client) => {
                if let Err(e) = client.database(db_name).drop(None).await {
                    return Err(Box::new(MongoDropDbErrorEnum::DatabaseDrop {
                        source: e,
                        where_was: WhereWas {
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                            location: *core::panic::Location::caller(),
                        },
                    }));
                }
                Ok(())
            }
        },
    }
}
