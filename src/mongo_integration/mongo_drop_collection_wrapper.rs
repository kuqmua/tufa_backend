use mongodb::bson::Document;
use mongodb::{options::ClientOptions, Client};

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_drop_collection_wrapper(
    mongo_url: &str,
    db_name: &str,
    db_collection_name: &str,
    check_if_collection_empty: bool,
) -> Result<bool, mongodb::error::Error> {
    let client_options = ClientOptions::parse(mongo_url).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database(db_name);
    let collection = db.collection::<Document>(db_collection_name);
    if !check_if_collection_empty {
        collection.drop(None).await?;
        return Ok(true)
    }
    let documents_number = collection.count_documents(None, None).await?;
    if documents_number == 0 {
        collection.drop(None).await?;
        Ok(true)
    } else {
        print_colorful_message(
            None,
            PrintType::WarningLow,
            file!().to_string(),
            line!().to_string(),
            "collection is not empty".to_string(),
        );
        Ok(false)
    }
}
