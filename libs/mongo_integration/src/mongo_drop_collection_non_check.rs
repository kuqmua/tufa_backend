use mongodb::{options::ClientOptions, Client};

#[tokio::main]
pub async fn mongo_drop_collection_non_check(
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
                    let collection = db.collection(db_collection_name);
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
