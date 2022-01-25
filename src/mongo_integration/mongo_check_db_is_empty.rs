use mongodb::{options::ClientOptions, Client};

#[derive(Debug)]
pub struct MongoCheckDbIsEmptyError {
    pub source: Box<MongoCheckDbIsEmptyErrorEnum>,
}

#[derive(Debug)]
pub enum MongoCheckDbIsEmptyErrorEnum {
    ClientOptionsParse(ClientOptionsParseError),
    ClientWithOptions(ClientWithOptionsError),
    ListCollectionNames(ListCollectionNamesError),
    NotEmpty(usize),
    DatabaseDrop(DatabaseDropError),
}

#[derive(Debug)]
pub struct ClientOptionsParseError {
    pub source: mongodb::error::Error,
}

#[derive(Debug)]
pub struct ClientWithOptionsError {
    pub source: mongodb::error::Error,
}

#[derive(Debug)]
pub struct DatabaseDropError {
    pub source: mongodb::error::Error,
}

#[derive(Debug)]
pub struct ListCollectionNamesError {
    pub source: mongodb::error::Error,
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_check_db_is_empty(
    mongo_url: &str,
    db_name: &str,
) -> Result<(), MongoCheckDbIsEmptyError> {
    //
    match ClientOptions::parse(mongo_url).await {
        Err(e) => {
            return Err(MongoCheckDbIsEmptyError {
                source: Box::new(
                    MongoCheckDbIsEmptyErrorEnum::ClientOptionsParse(
                        ClientOptionsParseError { source: e },
                    ),
                ),
            })
        }
        Ok(client_options) => {
            match Client::with_options(client_options) {
                Err(e) => {
                    return Err(MongoCheckDbIsEmptyError {
                        source: Box::new(
                            MongoCheckDbIsEmptyErrorEnum::ClientWithOptions(
                                ClientWithOptionsError { source: e },
                            ),
                        ),
                    })
                }
                Ok(client) => {
                    match client.database(db_name).list_collection_names(None).await {
                        Err(e) => {
                            return Err(MongoCheckDbIsEmptyError {
                                source: Box::new(
                                    MongoCheckDbIsEmptyErrorEnum::ListCollectionNames(
                                        ListCollectionNamesError { source: e },
                                    ),
                                ),
                            })
                        },
                        Ok(documents_number) => {
                            if !documents_number.is_empty() {
                                return Err(MongoCheckDbIsEmptyError {
                                    source: Box::new(
                                        MongoCheckDbIsEmptyErrorEnum::NotEmpty(documents_number.len()),
                                    ),
                                });
                            }
                            Ok(())
                        },
                    }
                }
            }
        },
    }
}
