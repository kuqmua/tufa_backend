use futures::stream::TryStreamExt;
use itertools::Itertools;
use mongodb::{bson::Document, Collection};

use crate::helpers::where_was::WhereWas;

#[derive(Debug)]
pub struct MongoGetDocumentsAsStringVectorError {
    pub source: Box<MongoGetDocumentsAsStringVectorErrorEnum>,
}

#[derive(Debug)]
pub enum MongoGetDocumentsAsStringVectorErrorEnum {
    CollectionAggregate {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    CursorTryNext {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    WrongBsonType {
        source: mongodb::bson::Bson,
        where_was: WhereWas,
    },
    NoKeyInDocument {
        source: String,
        where_was: WhereWas,
    },
}

//not a good solution, rewrite later
impl From<mongodb::error::Error> for MongoGetDocumentsAsStringVectorError {
    fn from(e: mongodb::error::Error) -> Self {
        MongoGetDocumentsAsStringVectorError {
            source: Box::new(MongoGetDocumentsAsStringVectorErrorEnum::CursorTryNext {
                source: e,
                where_was: WhereWas {
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            }),
        }
    }
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn mongo_get_documents_as_string_vector(
    collection: Collection<Document>,
    db_collection_document_field_name_handle: &str,
    option_aggregation: Option<Document>,
) -> Result<Vec<String>, MongoGetDocumentsAsStringVectorError> {
    match collection.aggregate(option_aggregation, None).await {
        Err(e) => Err(MongoGetDocumentsAsStringVectorError {
            source: Box::new(
                MongoGetDocumentsAsStringVectorErrorEnum::CollectionAggregate {
                    source: e,
                    where_was: WhereWas {
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                },
            ),
        }),
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
                                    where_was: WhereWas {
                                        file: file!(),
                                        line: line!(),
                                        column: column!(),
                                    },
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
                                        where_was: WhereWas {
                                            file: file!(),
                                            line: line!(),
                                            column: column!(),
                                        },
                                    },
                                ),
                            });
                        }
                    },
                }
            }
            let unique_vec = vec_of_strings.into_iter().unique().collect();
            Ok(unique_vec)
        }
    }
}
