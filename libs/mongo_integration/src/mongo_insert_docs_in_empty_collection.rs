use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    // options::CreateCollectionOptions,
    Client,
};
// This trait is required to use `try_next()` on the cursor
use futures::stream::TryStreamExt;
// use mongodb::options::FindOptions;

#[tokio::main]
pub async fn mongo_insert_docs_in_empty_collection(
    mongo_url: &str,
    db_name_handle: &str,
    db_collection_handle: &str,
    vec_of_values: Vec<&str>,
) -> mongodb::error::Result<()> {
    let client_options = ClientOptions::parse(mongo_url).await?;
    let client_result = Client::with_options(client_options);
    match client_result {
        Ok(client) => {
            let db = client.database(db_name_handle);
            let collection = db.collection(&db_collection_handle);
            let documents_number_result = collection.count_documents(None, None).await;
            match documents_number_result {
                Ok(documents_number) => {
                    if documents_number > 0 {
                        let docs = vec![doc! { "link_part": "1984" }];
                        let dd = collection.insert_many(docs, None).await;
                        match dd {
                            Ok(_) => println!("@@@@"),
                            Err(e) => println!("####, {:#?}", e),
                        }
                    } else {
                        println!("collection is not empty, docs did not inserted");
                    }
                }
                Err(e) => println!("####, {:#?}", e),
            }
        }
        Err(e) => {
            println!("no client , {:#?}", e);
        }
    }
    Ok(())
}
