use mongodb::{options::ClientOptions, Client};

#[derive(Debug)]
pub struct MongoCheckDbIsEmptyError {
    pub source: Box<MongoCheckDbIsEmptyErrorEnum>,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
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
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug)]
pub struct ClientWithOptionsError {
    pub source: mongodb::error::Error,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug)]
pub struct ListCollectionNamesError {
    pub source: mongodb::error::Error,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug)]
pub struct DatabaseDropError {
    pub source: mongodb::error::Error,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_check_db_is_empty(
    mongo_url: &str,
    db_name: &str,
) -> Result<(), MongoCheckDbIsEmptyError> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => Err(MongoCheckDbIsEmptyError {
            source: Box::new(MongoCheckDbIsEmptyErrorEnum::ClientOptionsParse(
                ClientOptionsParseError {
                    source: e,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            )),
            file: file!(),
            line: line!(),
            column: column!(),
        }),
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => Err(MongoCheckDbIsEmptyError {
                source: Box::new(MongoCheckDbIsEmptyErrorEnum::ClientWithOptions(
                    ClientWithOptionsError {
                        source: e,
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                )),
                file: file!(),
                line: line!(),
                column: column!(),
            }),
            Ok(client) => match client.database(db_name).list_collection_names(None).await {
                Err(e) => Err(MongoCheckDbIsEmptyError {
                    source: Box::new(MongoCheckDbIsEmptyErrorEnum::ListCollectionNames(
                        ListCollectionNamesError {
                            source: e,
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    )),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                }),
                Ok(documents_number) => {
                    if !documents_number.is_empty() {
                        return Err(MongoCheckDbIsEmptyError {
                            source: Box::new(MongoCheckDbIsEmptyErrorEnum::NotEmpty(
                                documents_number.len(),
                            )),
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        });
                    }
                    Ok(())
                }
            },
        },
    }
}
