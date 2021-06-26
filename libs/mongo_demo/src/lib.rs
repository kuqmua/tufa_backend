use mongodb::{bson::doc, options::ClientOptions, Client};
#[tokio::main]
pub async fn mongo_main() -> mongodb::error::Result<()> {
    //Parse your connection string into an options struct
    //mongodb+srv:<username>:<password>@<cluster-url>/test?w=majority
    let mut client_options =
        ClientOptions::parse("mongodb://root:rootpassword@localhost:27017").await?;
    //Manually set an option
    client_options.app_name = Some("Rust Demo".to_string());
    //Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    //Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected successfully.");
    //List the names of the databases in that cluster
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    Ok(())
}
