use mongodb::{options::ClientOptions, Client, Database};

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[tokio::main]
pub async fn mongo_drop_db(
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
                    result_flag = true;
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
