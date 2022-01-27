use mongodb::bson::Document;
use mongodb::{options::ClientOptions, Client};

#[derive(Debug)]
pub struct MongoCheckCollectionIsEmptyError {
    pub source: Box<MongoCheckCollectionIsEmptyErrorEnum>,
    line: String
}

#[derive(Debug)]
pub enum MongoCheckCollectionIsEmptyErrorEnum {
    ClientOptionsParse(ClientOptionsParseError),
    ClientWithOptions(ClientWithOptionsError),
    CountDocuments(CountDocumentsError),
    NotEmpty(u64),
}

#[derive(Debug)]
pub struct ClientOptionsParseError {
    pub source: mongodb::error::Error,
    line: String
}

#[derive(Debug)]
pub struct ClientWithOptionsError {
    pub source: mongodb::error::Error,
    line: String
}

#[derive(Debug)]
pub struct CountDocumentsError {
    pub source: mongodb::error::Error,
    line: String
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_check_collection_is_empty(
    mongo_url: &str,
    db_name: &str,
    db_collection_name: &str,
) -> Result<(), MongoCheckCollectionIsEmptyError> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => {
            return Err(MongoCheckCollectionIsEmptyError {
                source: Box::new(MongoCheckCollectionIsEmptyErrorEnum::ClientOptionsParse(
                    ClientOptionsParseError { source: e,line: format!("{} {}", line!().to_string(), file!().to_string()) },
                )),
                line: format!("{} {}", line!().to_string(), file!().to_string())
            })
        }
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => {
                return Err(MongoCheckCollectionIsEmptyError {
                    source: Box::new(MongoCheckCollectionIsEmptyErrorEnum::ClientWithOptions(
                        ClientWithOptionsError { source: e, line: format!("{} {}", line!().to_string(), file!().to_string()) },
                    )),
                    line: format!("{} {}", line!().to_string(), file!().to_string())
                })
            }
            Ok(client) => {
                match client
                    .database(db_name)
                    .collection::<Document>(db_collection_name)
                    .count_documents(None, None)
                    .await
                {
                    Err(e) => {
                        return Err(MongoCheckCollectionIsEmptyError {
                            source: Box::new(MongoCheckCollectionIsEmptyErrorEnum::CountDocuments(
                                CountDocumentsError { 
                                    source: e, 
                                    line: format!("{} {}", line!().to_string(), file!().to_string())
                                 },
                            )),
                            line: format!("{} {}", line!().to_string(), file!().to_string())
                        })
                    }
                    Ok(documents_number) => {
                        if documents_number > 0 {
                            return Err(MongoCheckCollectionIsEmptyError {
                                source: Box::new(MongoCheckCollectionIsEmptyErrorEnum::NotEmpty(
                                    documents_number,
                                )),
                                line: format!("{} {}", line!().to_string(), file!().to_string())
                            });
                        }
                        Ok(())
                    }
                }
            }
        },
    }
}
