use mongodb::bson::Document;
use mongodb::{options::ClientOptions, Client};
use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_check_collection_is_empty(
    mongo_url: &str,
    db_name_handle: &str,
    db_collection_handle: &str,
) -> Result<Option<bool>, mongodb::error::Error> {
    let client_options = ClientOptions::parse(mongo_url).await?;
    let client_result = Client::with_options(client_options);
    let result_flag: Option<bool>;
    match client_result {
        Ok(client) => {
            //declare db name. there is no create db method in mongo
            let db = client.database(db_name_handle);
            let mut needed_db_collection: Option<String> = None;
            for collection_name in db.list_collection_names(None).await? {
                if collection_name == *db_collection_handle {
                    needed_db_collection = Some(collection_name);
                }
            }
            match needed_db_collection {
                Some(collection_name) => {
                    let collection = db.collection::<Document>(&collection_name);
                    let documents_number_result = collection.count_documents(None, None).await;
                    match documents_number_result {
                        Ok(documents_number) => {
                            if documents_number == 0 {
                                result_flag = Some(true)
                            } else {
                                result_flag = Some(false)
                            }
                        }
                        Err(e) => {
                            result_flag = None;
                            print_colorful_message(
                                None,
                                PrintType::WarningHigh,
                                file!().to_string(),
                                line!().to_string(),
                                format!("documents_number_result error, {:#?}", e),
                            );
                        }
                    }
                }
                None => {
                    result_flag = None;
                    print_colorful_message(
                        None,
                        PrintType::WarningLow,
                        file!().to_string(),
                        line!().to_string(),
                        "needed_db_collection is None".to_string(),
                    );
                }
            }
        }
        Err(e) => {
            result_flag = None;
            print_colorful_message(
                None,
                PrintType::WarningLow,
                file!().to_string(),
                line!().to_string(),
                format!("client_result error, {:#?}", e),
            );
        }
    }
    Ok(result_flag)
}
