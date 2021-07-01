use futures::stream::TryStreamExt;
use mongodb::{options::ClientOptions, Client};

#[tokio::main]
pub async fn mongo_get_provider_link_parts(
    mongo_url: &str,
    db_name_handle: &str,
    db_collection_handle: &str,
    db_collection_key_handle: &str,
) -> Result<Option<Vec<String>>, mongodb::error::Error> {
    println!("BIG F");
    let client_options = ClientOptions::parse(mongo_url).await?;
    let client_result = Client::with_options(client_options);
    let mut option_vec_of_strings: Option<Vec<String>> = None;
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
                    let collection = db.collection(&collection_name);
                    let documents_number_result = collection.count_documents(None, None).await;
                    match documents_number_result {
                        Ok(documents_number) => {
                            if documents_number > 0 {
                                println!("collection.count_documents {}", documents_number);
                                let cursor_result = collection.find(None, None).await;
                                match cursor_result {
                                    Ok(mut cursor) => {
                                        let mut vec_of_strings: Vec<String> = Vec::new();
                                        while let Some(document) = cursor.try_next().await? {
                                            let bson_option =
                                                document.get(db_collection_key_handle);
                                            match bson_option {
                                                Some(bson_handle) => {
                                                    println!(
                                                        "link_part: {}",
                                                        bson_handle.to_string()
                                                    );
                                                    vec_of_strings.push(bson_handle.to_string())
                                                }
                                                None => {
                                                    println!(
                                                        "no db_collection_key_handle: {}",
                                                        db_collection_key_handle
                                                    );
                                                }
                                            }
                                        }
                                        option_vec_of_strings = Some(vec_of_strings)
                                    }
                                    Err(e) => println!(
                                        "(todo change this print)  collection.find, {:#?}",
                                        e
                                    ),
                                }
                            } else {
                                println!("documents_number is {}", documents_number)
                            }
                        }
                        Err(e) => println!(
                            "(todo change this print) collection.count_documents, {:#?}",
                            e
                        ),
                    }
                }
                None => {
                    println!("(todo change this print) no such collection");
                }
            }
        }
        Err(e) => {
            println!("(todo change this print) no client , {:#?}", e);
        }
    }
    Ok(option_vec_of_strings)
}
