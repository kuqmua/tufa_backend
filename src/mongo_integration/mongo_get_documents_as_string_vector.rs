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
            vec_of_strings.push(bson_handle.to_string());
        }
    }
    Ok(vec_of_strings)
}
