use futures::stream::TryStreamExt;
use mongodb::{bson::Document, Collection};

pub async fn mongo_get_documents_as_string_vector(
    collection: Collection<Document>,
    db_collection_document_field_name_handle: &str,
    option_aggregation_stage_1_get_docs_in_random_order_with_limit: Option<Document>,
) -> Result<Vec<String>, mongodb::error::Error> {
    let mut cursor = collection
        .aggregate(
            option_aggregation_stage_1_get_docs_in_random_order_with_limit,
            None,
        )
        .await?;
    let mut vec_of_strings: Vec<String> = Vec::new();
    while let Some(document) = cursor.try_next().await? {
        let bson_option = document.get(db_collection_document_field_name_handle);
        if let Some(bson_handle) = bson_option {
            match bson_handle {
                mongodb::bson::Bson::Double(_) => todo!(),
                mongodb::bson::Bson::String(value) => vec_of_strings.push(value.to_string()),
                mongodb::bson::Bson::Array(_) => todo!(),
                mongodb::bson::Bson::Document(_) => todo!(),
                mongodb::bson::Bson::Boolean(_) => todo!(),
                mongodb::bson::Bson::Null => todo!(),
                mongodb::bson::Bson::RegularExpression(_) => todo!(),
                mongodb::bson::Bson::JavaScriptCode(_) => todo!(),
                mongodb::bson::Bson::JavaScriptCodeWithScope(_) => todo!(),
                mongodb::bson::Bson::Int32(_) => todo!(),
                mongodb::bson::Bson::Int64(_) => todo!(),
                mongodb::bson::Bson::Timestamp(_) => todo!(),
                mongodb::bson::Bson::Binary(_) => todo!(),
                mongodb::bson::Bson::ObjectId(_) => todo!(),
                mongodb::bson::Bson::DateTime(_) => todo!(),
                mongodb::bson::Bson::Symbol(_) => todo!(),
                mongodb::bson::Bson::Decimal128(_) => todo!(),
                mongodb::bson::Bson::Undefined => todo!(),
                mongodb::bson::Bson::MaxKey => todo!(),
                mongodb::bson::Bson::MinKey => todo!(),
                mongodb::bson::Bson::DbPointer(_) => todo!(),
            }
        }
    }
    Ok(vec_of_strings)
}
