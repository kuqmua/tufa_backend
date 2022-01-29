use futures::stream::TryStreamExt;
use mongodb::{bson::Document, Collection};

#[derive(Debug)]
pub struct MongoGetDocumentsAsStringVectorError {
    pub source: Box<MongoGetDocumentsAsStringVectorErrorEnum>,
    line: String,
}

#[derive(Debug)]
pub enum MongoGetDocumentsAsStringVectorErrorEnum {
    CollectionAggregate(CollectionAggregateError),
    CursorTryNext(CursorTryNextError),
    WrongBsonType(mongodb::bson::Bson),
    NoKeyInDocument(String),
}

//not a good solution, rewrite later
impl From<mongodb::error::Error> for MongoGetDocumentsAsStringVectorError {
    fn from(e: mongodb::error::Error) -> Self {
        MongoGetDocumentsAsStringVectorError {
            source: Box::new(MongoGetDocumentsAsStringVectorErrorEnum::CursorTryNext(
                CursorTryNextError {
                    source: e,
                    line: format!("{}:{}:{}", file!(), line!(), column!()),
                },
            )),
            line: format!("{}:{}:{}", file!(), line!(), column!()),
        }
    }
}

#[derive(Debug)]
pub struct CollectionAggregateError {
    pub source: mongodb::error::Error,
    line: String,
}

#[derive(Debug)]
pub struct CursorTryNextError {
    pub source: mongodb::error::Error,
    line: String,
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
                    MongoGetDocumentsAsStringVectorErrorEnum::CollectionAggregate(
                        CollectionAggregateError {
                            source: e,
                            line: format!("{}:{}:{}", file!(), line!(), column!()),
                        },
                    ),
                ),
                line: format!("{}:{}:{}", file!(), line!(), column!()),
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
                                MongoGetDocumentsAsStringVectorErrorEnum::NoKeyInDocument(
                                    db_collection_document_field_name_handle.to_string(),
                                ),
                            ),
                            line: format!("{}:{}:{}", file!(), line!(), column!()),
                        })
                    }
                    Some(bson_handle) => match bson_handle {
                        mongodb::bson::Bson::String(value) => {
                            vec_of_strings.push(value.to_string());
                        }
                        other_bson_type => {
                            return Err(MongoGetDocumentsAsStringVectorError {
                                source: Box::new(
                                    MongoGetDocumentsAsStringVectorErrorEnum::WrongBsonType(
                                        other_bson_type.clone(),
                                    ),
                                ),
                                line: format!("{}:{}:{}", file!(), line!(), column!()),
                            });
                        }
                    },
                }
            }
            Ok(vec_of_strings)
        }
    }
}
