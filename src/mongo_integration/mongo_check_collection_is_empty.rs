use mongodb::bson::Document;
use mongodb::options::ClientOptions;
use mongodb::Client;
use tufa_common::where_was::WhereWas;

#[derive(Debug)]
pub enum MongoCheckCollectionIsEmptyErrorEnum {
    ClientOptionsParse {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    ClientWithOptions {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    CountDocuments {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    NotEmpty {
        source: u64,
        where_was: WhereWas,
    },
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn mongo_check_collection_is_empty(
    mongo_url: &str,
    db_name: &str,
    db_collection_name: &str,
) -> Result<(), Box<MongoCheckCollectionIsEmptyErrorEnum>> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => Err(Box::new(
            MongoCheckCollectionIsEmptyErrorEnum::ClientOptionsParse {
                source: e,
                where_was: WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    location: *core::panic::Location::caller(),
                },
            },
        )),
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => Err(Box::new(
                MongoCheckCollectionIsEmptyErrorEnum::ClientWithOptions {
                    source: e,
                    where_was: WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                },
            )),
            Ok(client) => {
                match client
                    .database(db_name)
                    .collection::<Document>(db_collection_name)
                    .count_documents(None, None)
                    .await
                {
                    Err(e) => Err(Box::new(
                        MongoCheckCollectionIsEmptyErrorEnum::CountDocuments {
                            source: e,
                            where_was: WhereWas {
                                time: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .expect("cannot convert time to unix_epoch"),
                                location: *core::panic::Location::caller(),
                            },
                        },
                    )),
                    Ok(documents_number) => {
                        if documents_number > 0 {
                            return Err(Box::new(MongoCheckCollectionIsEmptyErrorEnum::NotEmpty {
                                source: documents_number,
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
                }
            }
        },
    }
}
