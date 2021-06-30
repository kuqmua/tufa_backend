use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client,
};

#[tokio::main]
pub async fn mongo_insert_docs_in_empty_collection(
    mongo_url: &str,
    db_name_handle: &str,
    db_collection_handle: &str,
    db_collection_document_field_name: &str,
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
                        println!("collection is not empty, docs did not inserted");
                    } else {
                        let mut docs: Vec<Document> = Vec::with_capacity(vec_of_values.len());
                        for value in &vec_of_values {
                            println!("value {}", value.len());
                            docs.push(doc! { db_collection_document_field_name: value });
                        }
                        let insert_many_result = collection.insert_many(docs, None).await;
                        match insert_many_result {
                            Ok(_) => {
                                println!(
                                    "successfull insertion {} elements to {} collection!",
                                    vec_of_values.len(),
                                    db_collection_handle
                                )
                            }
                            Err(e) => println!("####, {:#?}", e),
                        }
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
