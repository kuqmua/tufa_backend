use futures::stream::TryStreamExt;
use mongodb::{bson::Document, Collection};

#[derive(Debug)]
pub struct MongoGetDocumentsAsStringVectorError {
    pub source: Box<MongoGetDocumentsAsStringVectorErrorEnum>,
}

#[derive(Debug)]
pub enum MongoGetDocumentsAsStringVectorErrorEnum {
    CollectionAggregate {
        source: mongodb::error::Error,
        file: &'static str,
        line: u32,
        column: u32,
    },
    CursorTryNext {
        source: mongodb::error::Error,
        file: &'static str,
        line: u32,
        column: u32,
    },
    WrongBsonType {
        source: mongodb::bson::Bson,
        file: &'static str,
        line: u32,
        column: u32,
    },
    NoKeyInDocument {
        source: String,
        file: &'static str,
        line: u32,
        column: u32,
    },
}

//not a good solution, rewrite later
impl From<mongodb::error::Error> for MongoGetDocumentsAsStringVectorError {
    fn from(e: mongodb::error::Error) -> Self {
        MongoGetDocumentsAsStringVectorError {
            source: Box::new(MongoGetDocumentsAsStringVectorErrorEnum::CursorTryNext {
                source: e,
                file: file!(),
                line: line!(),
                column: column!(),
            }),
        }
    }
}

pub async fn mongo_get_documents_as_string_vector(
    collection: Collection<Document>,
    db_collection_document_field_name_handle: &str,
    option_aggregation: Option<Document>,
) -> Result<Vec<String>, MongoGetDocumentsAsStringVectorError> {
    match collection.aggregate(option_aggregation, None).await {
        Err(e) => {
            return Err(MongoGetDocumentsAsStringVectorError {
                source: Box::new(
                    MongoGetDocumentsAsStringVectorErrorEnum::CollectionAggregate {
                        source: e,
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                ),
            });
        }
        Ok(mut cursor) => {
            let mut vec_of_strings: Vec<String> = Vec::new();
            //dont know yet how to convert this expression into for explicit way
            while let Some(document) = cursor.try_next().await? {
                match document.get(db_collection_document_field_name_handle) {
                    None => {
                        return Err(MongoGetDocumentsAsStringVectorError {
                            source: Box::new(
                                MongoGetDocumentsAsStringVectorErrorEnum::NoKeyInDocument {
                                    source: db_collection_document_field_name_handle.to_string(),
                                    file: file!(),
                                    line: line!(),
                                    column: column!(),
                                },
                            ),
                        })
                    }
                    Some(bson_handle) => match bson_handle {
                        mongodb::bson::Bson::String(value) => {
                            vec_of_strings.push(value.to_string());
                        }
                        other_bson_type => {
                            return Err(MongoGetDocumentsAsStringVectorError {
                                source: Box::new(
                                    MongoGetDocumentsAsStringVectorErrorEnum::WrongBsonType {
                                        source: other_bson_type.clone(),
                                        file: file!(),
                                        line: line!(),
                                        column: column!(),
                                    },
                                ),
                            });
                        }
                    },
                }
            }
            Ok(vec_of_strings)
        }
    }
}
