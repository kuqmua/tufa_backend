use mongodb::bson::Document;
use mongodb::{options::ClientOptions, Client};

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

//it will fail on runtime if remove #[tokio::main]
#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_drop_collection(
    mongo_url: &str,
    db_name: &str,
    db_collection_name: &str,
) -> Result<bool, mongodb::error::Error> {
    let result_flag: bool;
    let client_options_result = ClientOptions::parse(mongo_url).await;
    match client_options_result {
        Ok(client_options) => {
            let client_result = Client::with_options(client_options);
            match client_result {
                Ok(client) => {
                    let db = client.database(db_name);
                    let collection = db.collection::<Document>(db_collection_name);
                    let collection_drop_result = collection.drop(None).await;
                    match collection_drop_result {
                        Ok(()) => result_flag = true,
                        Err(e) => {
                            result_flag = false;
                            print_colorful_message(
                                None,
                                PrintType::Error,
                                file!().to_string(),
                                line!().to_string(),
                                format!("collection_drop_result error {:#?}", e),
                            );
                        }
                    }
                }
                Err(e) => {
                    result_flag = false;
                    print_colorful_message(
                        None,
                        PrintType::Error,
                        file!().to_string(),
                        line!().to_string(),
                        format!("client_result error {:#?}", e),
                    );
                }
            }
        }
        Err(e) => {
            result_flag = false;
            print_colorful_message(
                None,
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                format!("client_options_result error {:#?}", e),
            );
        }
    }
    Ok(result_flag)
}
