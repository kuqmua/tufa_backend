use mongodb::{options::ClientOptions, Client};

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

//it will fail on runtime if remove #[tokio::main]
#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[tokio::main] //using different (old) tokio runtime 0.2.25
pub async fn mongo_drop_collection_wrapper(
    mongo_url: &str,
    db_name: &str,
    db_collection_name: &str,
    check_if_collection_empty: bool,
) -> Result<bool, mongodb::error::Error> {
    let result_flag: bool;
    let client_options_result = ClientOptions::parse(mongo_url).await;
    match client_options_result {
        Ok(client_options) => {
            let client_result = Client::with_options(client_options);
            match client_result {
                Ok(client) => {
                    let db = client.database(db_name);
                    let collection = db.collection(db_collection_name);
                    if check_if_collection_empty {
                        let documents_number_result = collection.count_documents(None, None).await;
                        match documents_number_result {
                            Ok(documents_number) => {
                                if documents_number > 0 {
                                    result_flag = false;
                                    print_colorful_message(
                                        None,
                                        PrintType::WarningLow,
                                        file!().to_string(),
                                        line!().to_string(),
                                        "collection is not empty".to_string(),
                                    );
                                } else {
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
                            }
                            Err(e) => {
                                result_flag = false;
                                print_colorful_message(
                                    None,
                                    PrintType::Error,
                                    file!().to_string(),
                                    line!().to_string(),
                                    format!("documents_number_result error {:#?}", e),
                                );
                            }
                        }
                    } else {
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
