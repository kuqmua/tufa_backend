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
        line: String,
    },
    CursorTryNext {
        source: mongodb::error::Error,
        line: String,
    },
    WrongBsonType {
        source: mongodb::bson::Bson,
        line: String,
    },
    NoKeyInDocument {
        source: String,
        line: String,
    },
}

//not a good solution, rewrite later
impl From<mongodb::error::Error> for MongoGetDocumentsAsStringVectorError {
    fn from(e: mongodb::error::Error) -> Self {
        MongoGetDocumentsAsStringVectorError {
            source: Box::new(MongoGetDocumentsAsStringVectorErrorEnum::CursorTryNext {
                source: e,
                line: format!("{}:{}:{}", file!(), line!(), column!()),
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
                        line: format!("{}:{}:{}", file!(), line!(), column!()),
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
                                    line: format!("{}:{}:{}", file!(), line!(), column!()),
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
                                        line: format!("{}:{}:{}", file!(), line!(), column!()),
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
