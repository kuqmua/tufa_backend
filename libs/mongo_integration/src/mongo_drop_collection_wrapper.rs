use mongodb::{options::ClientOptions, Client};

#[tokio::main]
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
                                    println!("collection is not empty");
                                } else {
                                    let collection_drop_result = collection.drop(None).await;
                                    match collection_drop_result {
                                        Ok(()) => {
                                            result_flag = true;
                                        }
                                        Err(e) => {
                                            result_flag = false;
                                            println!("collection_drop_result error {:#?}", e)
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                println!("documents_number_result error {:#?}", e);
                                let collection_drop_result = collection.drop(None).await;
                                match collection_drop_result {
                                    Ok(()) => {
                                        result_flag = true;
                                    }
                                    Err(e) => {
                                        result_flag = false;
                                        println!("collection_drop_result error {:#?}", e)
                                    }
                                }
                            }
                        }
                    } else {
                        let collection_drop_result = collection.drop(None).await;
                        match collection_drop_result {
                            Ok(()) => {
                                result_flag = true;
                            }
                            Err(e) => {
                                result_flag = false;
                                println!("collection_drop_result error {:#?}", e)
                            }
                        }
                    }
                }
                Err(e) => {
                    result_flag = false;
                    println!("client_result error {:#?}", e)
                }
            }
        }
        Err(e) => {
            result_flag = false;
            println!("client_options_result error {:#?}", e)
        }
    }
    Ok(result_flag)
}
