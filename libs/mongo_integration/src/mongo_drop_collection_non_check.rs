use mongodb::{options::ClientOptions, Client};

#[tokio::main]
pub async fn mongo_drop_collection_non_check(
    mongo_url: &str,
    db_name: &str,
    db_collection_name: &str,
) -> Result<bool, mongodb::error::Error> {
    let client_options = ClientOptions::parse(mongo_url).await?;
    let client_result = Client::with_options(client_options);
    let result_flag: bool;
    match client_result {
        Ok(client) => {
            result_flag = true;
            let db = client.database(db_name);
            let collection = db.collection(db_collection_name);
            collection.drop(None).await?;
        }
        Err(e) => {
            result_flag = false;
            println!("client_result error {:#?}", e)
        }
    }
    Ok(result_flag)
}
