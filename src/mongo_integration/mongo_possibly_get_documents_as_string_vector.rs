use futures::stream::TryStreamExt;
use mongodb::{
    bson::{self, Document},
    Collection,
};

pub async fn mongo_possibly_get_documents_as_string_vector(
    collection: Collection<Document>,
    db_collection_document_field_name_handle: &str,
    option_aggregation_stage_1_get_docs_in_random_order_with_limit: Option<Document>,
) -> Result<Vec<String>, mongodb::error::Error> {
    let cursor_result = collection
        .aggregate(
            option_aggregation_stage_1_get_docs_in_random_order_with_limit,
            None,
        )
        .await;
    match cursor_result {
        Ok(mut cursor) => {
            let mut vec_of_strings: Vec<String> = Vec::new();
            while let Some(document) = cursor.try_next().await? {
                let bson_option = document.get(db_collection_document_field_name_handle);
                match bson_option {
                    Some(bson_handle) => match bson_handle {
                        bson::Bson::String(stringified_bson) => {
                            vec_of_strings.push(stringified_bson.to_string())
                        }
                        _ => {
                            println!("(todo change this print) different mongo type")
                        }
                    },
                    None => {
                        println!(
                            "no db_collection_document_field_name_handle: {}",
                            db_collection_document_field_name_handle
                        );
                    }
                }
            }
            if !vec_of_strings.is_empty() {
                return Ok(vec_of_strings);
            } else {
                return Ok(Vec::new());
            }
        }
        Err(e) => {
            println!("(todo change this print)  collection.find, {:#?}", e);
            return Ok(Vec::new());
        }
    }
}
