use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    // options::CreateCollectionOptions,
    Client,
};
// This trait is required to use `try_next()` on the cursor
use futures::stream::TryStreamExt;
// use mongodb::options::FindOptions;
pub mod mongo_insert_docs_in_empty_collection;

#[tokio::main]
pub async fn mongo_integration() -> mongodb::error::Result<()> {
    //mongodb://root:rootpassword@localhost:27017
    //mongodb+srv://mongodbcloudlogin:mongodbcloudpassword@tufa-mongo.y2xob.mongodb.net/myFirstDatabase?retryWrites=true&w=majority
    let mut client_options =
        ClientOptions::parse("mongodb+srv://mongodbcloudlogin:mongodbcloudpassword@tufa-mongo.y2xob.mongodb.net/myFirstDatabase?retryWrites=true&w=majority").await?;
    // client_options.app_name = Some("Rust Demo".to_string());
    let client_result = Client::with_options(client_options);
    let db_name_handle = "testdatabase";
    let db_collection_handle = "testcollection";
    let db_collection_key_handle = "link_part";
    let mut vec_of_strings: Vec<String> = Vec::new();
    match client_result {
        Ok(client) => {
            println!("client");
            //declare db name. there is no create db method in mongo
            let db = client.database(db_name_handle);
            let mut needed_db_collection: Option<String> = None;
            for collection_name in db.list_collection_names(None).await? {
                if collection_name == *db_collection_handle {
                    println!("+++++++++++++++++++={}", collection_name);
                    needed_db_collection = Some(collection_name);
                }
            }
            match needed_db_collection {
                Some(collection_name) => {
                    let collection = db.collection(&collection_name);
                    // let docs = vec![doc! { "link_part": "1984" }];
                    // let dd = collection.insert_many(docs, None).await;
                    // match dd {
                    //     Ok(_) => println!("@@@@"),
                    //     Err(e) => println!("####, {:#?}", e),
                    // }
                    let documents_number_result = collection.count_documents(None, None).await;
                    match documents_number_result {
                        Ok(documents_number) => {
                            if documents_number > 0 {
                                println!("collection.count_documents {}", documents_number);
                                // db.run_command(doc! {"ping": 1}, None).await?;
                                // let filter = doc! { "author": "George Orwell" };
                                // let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
                                let cursor_result = collection.find(None, None).await;
                                match cursor_result {
                                    Ok(mut cursor) => {
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
                                        // collection.drop(None).await?;
                                    }
                                    Err(e) => println!("####, {:#?}", e),
                                }
                            } else {
                                println!("documents_number is {}", documents_number)
                            }
                        }
                        Err(e) => println!("####, {:#?}", e),
                    }
                }
                None => {
                    //create collection?
                    let dd = db.create_collection("testcollection", None).await;
                    match dd {
                        Ok(()) => {
                            let collection = db.collection("test collection");
                            let docs = vec![doc! { "link_part": "1984" }];
                            let dd = collection.insert_many(docs, None).await;
                            match dd {
                                Ok(_) => println!("@@@@"),
                                Err(e) => println!("####, {:#?}", e),
                            }
                            let bbbb = collection.count_documents(None, None).await?;
                            println!("collection.count_documents {}", bbbb);
                            db.run_command(doc! {"ping": 1}, None).await?;
                            // let filter = doc! { "author": "George Orwell" };
                            // let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
                            let mut cursor = collection.find(None, None).await?;
                            // Iterate over the results of the cursor.
                            while let Some(book) = cursor.try_next().await? {
                                let reslt = book.get("link_part");
                                match reslt {
                                    Some(title) => {
                                        //there is an _id field
                                        println!("link_part: {}", title.to_string());
                                    }
                                    None => {
                                        println!("no link_part");
                                    }
                                }
                            }
                            collection.drop(None).await?;
                        }
                        Err(e) => {
                            println!("####, {:#?}", e);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("no client , {:#?}", e);
        }
    }
    println!("vec_of_strings {:#?}", vec_of_strings);
    Ok(())
}
