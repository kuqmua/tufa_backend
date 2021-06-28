use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    options::CreateCollectionOptions,
    Client,
};
#[tokio::main]
pub async fn mongo_integration() -> mongodb::error::Result<()> {
    //Parse your connection string into an options struct
    //mongodb://root:rootpassword@localhost:27017
    //mongodb+srv://mongodbcloudlogin:mongodbcloudpassword@tufa-mongo.y2xob.mongodb.net/myFirstDatabase?retryWrites=true&w=majority
    let mut client_options =
        ClientOptions::parse("mongodb://root:rootpassword@localhost:27017").await?;
    //Manually set an option
    client_options.app_name = Some("Rust Demo".to_string());
    //Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    //Ping the server to see if you can connect to the cluster
    let db = client.database("admin");
    println!("1");
    let dd = db.create_collection("test collection", None).await;
    match dd {
        Ok(()) => {
            println!("@@@@");
        }
        Err(e) => {
            println!("####, {:#?}", e);
        }
    }
    let collection = db.collection("test collection");
    println!("2");
    let docs = vec![
        doc! { "title": "1984", "author": "George Orwell" },
        doc! { "title": "Animal Farm", "author": "George Orwell" },
        doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];
    println!("3");
    // Insert some documents into the "mydb.books" collection.
    collection.insert_many(docs, None).await?;
    println!("4");
    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }
    db.run_command(doc! {"ping": 1}, None).await?;
    println!("Connected successfully.");
    //List the names of the databases in that cluster
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    Ok(())
}
