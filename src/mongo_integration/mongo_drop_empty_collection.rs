use mongodb::bson::Document;
use mongodb::{options::ClientOptions, Client};

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_drop_empty_collection(
    mongo_url: &str,
    db_name: &str,
    db_collection_name: &str,
) -> Result<bool, mongodb::error::Error> {
    let client_options = ClientOptions::parse(mongo_url).await?;
    let client = Client::with_options(client_options)?;
    let collection = client
        .database(db_name)
        .collection::<Document>(db_collection_name);
    let documents_number = collection.count_documents(None, None).await?;
    if documents_number > 0 {
        print_colorful_message(
            None,
            PrintType::WarningLow,
            file!().to_string(),
            line!().to_string(),
            "collection is not empty".to_string(),
        );
        Ok(false)
    } else {
        collection.drop(None).await?;
        Ok(true)
    }
}
