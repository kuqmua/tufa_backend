use mongodb::bson::Document;
use mongodb::options::ClientOptions;
use mongodb::Client;
use mongodb::Collection;
use tufa_common::common::where_was::WhereWas;

#[derive(Debug)]
pub enum MongoDropEmptyCollectionErrorEnum {
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
    DatabaseDrop {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
}

pub async fn mongo_drop_empty_collection(
    mongo_url: &str,
    db_name: &str,
    db_collection_name: &str,
) -> Result<(), Box<MongoDropEmptyCollectionErrorEnum>> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => Err(Box::new(
            MongoDropEmptyCollectionErrorEnum::ClientOptionsParse {
                source: e,
                where_was: WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    file: String::from(file!()),
                    line: line!(),
                    column: column!(),
                    git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                },
            },
        )),
        Ok(client_options) => {
            match Client::with_options(client_options) {
                Err(e) => Err(Box::new(
                    MongoDropEmptyCollectionErrorEnum::ClientWithOptions {
                        source: e,
                        where_was: WhereWas {
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                            file: String::from(file!()),
                            line: line!(),
                            column: column!(),
                            git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                        },
                    },
                )),
                Ok(client) => {
                    let collection: Collection<Document> =
                        client.database(db_name).collection(db_collection_name);
                    match collection.count_documents(None, None).await {
                    Err(e) => Err(Box::new(
                        MongoDropEmptyCollectionErrorEnum::CountDocuments {
                            source: e,
            where_was: WhereWas {
                time: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("cannot convert time to unix_epoch"),
                file: String::from(file!()),
                line: line!(),
                column: column!(),
                git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
            },
                        },
                    )),
                    Ok(documents_number) => {
                        if documents_number > 0 {
                            Err(Box::new(MongoDropEmptyCollectionErrorEnum::NotEmpty {
                                source: documents_number,
                                where_was:                 WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    file: String::from(file!()),
                    line: line!(),
                    column: column!(),
                    git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                },
                            }))
                        } else {
                            if let Err(e) = collection.drop(None).await {
                                return Err(Box::new(
                                    MongoDropEmptyCollectionErrorEnum::DatabaseDrop {
                                        source: e,
                                        where_was:                 WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    file: String::from(file!()),
                    line: line!(),
                    column: column!(),
                    git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                },
                                    },
                                ));
                            }
                            Ok(())
                        }
                    }
                }
                }
            }
        }
    }
}
