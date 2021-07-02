use futures::stream::TryStreamExt;
use mongodb::{bson, options::ClientOptions, Client};

#[tokio::main]
pub async fn mongo_get_provider_link_parts_as_bson_string(
    mongo_url: &str,
    db_name_handle: &str,
    db_collection_handle: &str,
    db_collection_key_handle: &str,
) -> Result<Vec<String>, mongodb::error::Error> {
    let client_options = ClientOptions::parse(mongo_url).await?;
    let client_result = Client::with_options(client_options);
    let vec_of_strings_to_return: Vec<String>;
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
                                                Some(bson_handle) => match bson_handle {
                                                    bson::Bson::String(stringified_bson) => {
                                                        vec_of_strings
                                                            .push(stringified_bson.to_string())
                                                    }
                                                    _ => {
                                                        println!("(todo change this print) different mongo type")
                                                    }
                                                },
                                                None => {
                                                    println!(
                                                        "no db_collection_key_handle: {}",
                                                        db_collection_key_handle
                                                    );
                                                }
                                            }
                                        }
                                        if vec_of_strings.len() > 0 {
                                            vec_of_strings_to_return = vec_of_strings
                                        } else {
                                            vec_of_strings_to_return = Vec::new()
                                        }
                                    }
                                    Err(e) => {
                                        vec_of_strings_to_return = Vec::new();
                                        println!(
                                            "(todo change this print)  collection.find, {:#?}",
                                            e
                                        )
                                    }
                                }
                            } else {
                                vec_of_strings_to_return = Vec::new();
                                println!("documents_number is {}", documents_number)
                            }
                        }
                        Err(e) => {
                            vec_of_strings_to_return = Vec::new();
                            println!(
                                "(todo change this print) collection.count_documents, {:#?}",
                                e
                            )
                        }
                    }
                }
                None => {
                    vec_of_strings_to_return = Vec::new();
                    println!("(todo change this print) no such collection");
                }
            }
        }
        Err(e) => {
            vec_of_strings_to_return = Vec::new();
            println!("(todo change this print) no client , {:#?}", e);
        }
    }
    Ok(vec_of_strings_to_return)
}
