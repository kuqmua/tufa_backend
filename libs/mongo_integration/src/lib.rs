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
pub async fn mongo_integration() -> mongodb::error::Result<()> {
    //mongodb://root:rootpassword@localhost:27017
    //mongodb+srv://mongodbcloudlogin:mongodbcloudpassword@tufa-mongo.y2xob.mongodb.net/myFirstDatabase?retryWrites=true&w=majority
    let mut client_options =
        ClientOptions::parse("mongodb+srv://mongodbcloudlogin:mongodbcloudpassword@tufa-mongo.y2xob.mongodb.net/myFirstDatabase?retryWrites=true&w=majority").await?;
    // client_options.app_name = Some("Rust Demo".to_string());
    let client_result = Client::with_options(client_options);
    match client_result {
        Ok(client) => {
            println!("client");
            //List the names of the databases in that cluster
            // let vec_of_db_names: Vec<String> = Vec::new();
            let mut needed_db_name: Option<String> = None;
            for db_name in client.list_database_names(None, None).await? {
                if db_name == *"testdatabase" {
                    needed_db_name = Some(db_name);
                }
            }
            match needed_db_name {
                Some(db_name) => {
                    let db = client.database(&db_name);
                    let mut needed_db_collection: Option<String> = None;
                    for collection_name in db.list_collection_names(None).await? {
                        println!("+++++++++++++++++++={}", collection_name);
                        if collection_name == *"testcollection" {
                            needed_db_collection = Some(collection_name);
                        }
                    }
                    match needed_db_collection {
                        Some(collection_name) => {
                            let collection = db.collection(&collection_name);
                            let docs = vec![doc! { "link_part": "1984" }];
                            let dd = collection.insert_many(docs, None).await;
                            match dd {
                                Ok(_) => println!("@@@@"),
                                Err(e) => println!("####, {:#?}", e),
                            }
                            let bbbb = collection.count_documents(None, None).await?;
                            println!("collection.count_documents {}", bbbb);
                            db.run_command(doc! {"ping": 1}, None).await?;
                            //
                            // let filter = doc! { "author": "George Orwell" };
                            // let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
                            let mut cursor = collection.find(None, None).await?;
                            // Iterate over the results of the cursor.
                            while let Some(book) = cursor.try_next().await? {
                                let reslt = book.get("link_part");
                                match reslt {
                                    Some(title) => {
                                        println!("link_part: {}", title.to_string());
                                        // let rss_struct_from_str_result: Result<RedditStructForParsing, serde_json::Error> =
                                        //     serde_json::from_str(&fetch_result_string);
                                    }
                                    None => {
                                        println!("no link_part");
                                    }
                                }
                            }
                            collection.drop(None).await?;
                        }
                        None => {
                            //create collection?
                            let dd = db.create_collection("testcollection", None).await;
                            match dd {
                                Ok(()) => {
                                    println!("@@@@");
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
                                    //
                                    // let filter = doc! { "author": "George Orwell" };
                                    // let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
                                    let mut cursor = collection.find(None, None).await?;
                                    // Iterate over the results of the cursor.
                                    while let Some(book) = cursor.try_next().await? {
                                        let reslt = book.get("link_part");
                                        match reslt {
                                            Some(title) => {
                                                println!("link_part: {}", title.to_string());
                                                // let rss_struct_from_str_result: Result<RedditStructForParsing, serde_json::Error> =
                                                //     serde_json::from_str(&fetch_result_string);
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
                None => {
                    // create db?
                    println!("create db?")
                }
            }
        }
        Err(e) => {
            println!("no client , {:#?}", e);
        }
    }

    Ok(())
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CommonProviderLinkPartMongoStruct {
    #[serde(rename = "_id", default)]
    pub id: Option<String>, //ObjectId("60db5fb500afe50b00c5a797")
    pub link_part: Option<String>,
}
