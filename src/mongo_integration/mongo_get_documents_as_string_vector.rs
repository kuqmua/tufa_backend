use std::fmt;

use futures::stream::TryStreamExt;
use mongodb::{bson::Document, Collection};

#[derive(Debug, BoxErrFromErrDerive, ImplDisplayDerive)]
pub struct MongoGetDocumentsAsStringVectorError {
    pub source: Box<MongoGetDocumentsAsStringVectorErrorEnum>,
}

#[derive(Debug, ImplFromForUpperStruct)]
pub enum MongoGetDocumentsAsStringVectorErrorEnum {
    MongoError(mongodb::error::Error),
    WrongBsonType(mongodb::bson::Bson),
    NoKeyInDocument(String),
}

pub async fn mongo_get_documents_as_string_vector(
    collection: Collection<Document>,
    db_collection_document_field_name_handle: &str,
    option_aggregation: Option<Document>,
) -> Result<Vec<String>, MongoGetDocumentsAsStringVectorError> {
    let mut cursor = collection.aggregate(option_aggregation, None).await?;
    let mut vec_of_strings: Vec<String> = Vec::new();
    while let Some(document) = cursor.try_next().await? {
        match document.get(db_collection_document_field_name_handle) {
            None => return Err(MongoGetDocumentsAsStringVectorError {
                source: Box::new(MongoGetDocumentsAsStringVectorErrorEnum::NoKeyInDocument(
                    db_collection_document_field_name_handle.to_string(),
                )),
            }),
            Some(bson_handle) => match bson_handle {
                mongodb::bson::Bson::String(value) => {
                    vec_of_strings.push(value.to_string());
                }
                other_bson_type => {
                    return Err(MongoGetDocumentsAsStringVectorError {
                        source: Box::new(MongoGetDocumentsAsStringVectorErrorEnum::WrongBsonType(
                            other_bson_type.clone(),
                        )),
                    });
                }
            },
        }
    }
    Ok(vec_of_strings)
}
