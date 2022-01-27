use mongodb::{options::ClientOptions, Client};

#[derive(Debug)]
pub struct MongoCheckDbIsEmptyError {
    pub source: Box<MongoCheckDbIsEmptyErrorEnum>,
    line: String,
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
    line: String,
}

#[derive(Debug)]
pub struct ClientWithOptionsError {
    pub source: mongodb::error::Error,
    line: String,
}

#[derive(Debug)]
pub struct ListCollectionNamesError {
    pub source: mongodb::error::Error,
    line: String,
}

#[derive(Debug)]
pub struct DatabaseDropError {
    pub source: mongodb::error::Error,
    line: String,
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_check_db_is_empty(
    mongo_url: &str,
    db_name: &str,
) -> Result<(), MongoCheckDbIsEmptyError> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => {
            return Err(MongoCheckDbIsEmptyError {
                source: Box::new(MongoCheckDbIsEmptyErrorEnum::ClientOptionsParse(
                    ClientOptionsParseError {
                        source: e,
                        line: format!("{} {}", line!().to_string(), file!().to_string()),
                    },
                )),
                line: format!("{} {}", line!().to_string(), file!().to_string()),
            })
        }
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => {
                return Err(MongoCheckDbIsEmptyError {
                    source: Box::new(MongoCheckDbIsEmptyErrorEnum::ClientWithOptions(
                        ClientWithOptionsError {
                            source: e,
                            line: format!("{} {}", line!().to_string(), file!().to_string()),
                        },
                    )),
                    line: format!("{} {}", line!().to_string(), file!().to_string()),
                })
            }
            Ok(client) => match client.database(db_name).list_collection_names(None).await {
                Err(e) => {
                    return Err(MongoCheckDbIsEmptyError {
                        source: Box::new(MongoCheckDbIsEmptyErrorEnum::ListCollectionNames(
                            ListCollectionNamesError {
                                source: e,
                                line: format!("{} {}", line!().to_string(), file!().to_string()),
                            },
                        )),
                        line: format!("{} {}", line!().to_string(), file!().to_string()),
                    })
                }
                Ok(documents_number) => {
                    if !documents_number.is_empty() {
                        return Err(MongoCheckDbIsEmptyError {
                            source: Box::new(MongoCheckDbIsEmptyErrorEnum::NotEmpty(
                                documents_number.len(),
                            )),
                            line: format!("{} {}", line!().to_string(), file!().to_string()),
                        });
                    }
                    Ok(())
                }
            },
        },
    }
}
