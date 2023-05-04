pub async fn mongo_drop_collection<'a>(
    mongo_url: &'a str,
    db_name: &'a str,
    db_collection_name: &'a str,
) -> Result<(), Box<tufa_common::repositories_types::tufa_server::mongo_integration::mongo_drop_collection::MongoDropCollectionErrorNamed<'a>>> {
    match mongodb::options::ClientOptions::parse(mongo_url).await {
        Err(e) => Err(Box::new(
            tufa_common::repositories_types::tufa_server::mongo_integration::mongo_drop_collection::MongoDropCollectionErrorNamed::MongoDB {
                mongodb: e,
                code_occurence: tufa_common::code_occurence!(),
            }
        )),
        Ok(client_options) => match mongodb::Client::with_options(client_options) {
            Err(e) => Err(Box::new(
                tufa_common::repositories_types::tufa_server::mongo_integration::mongo_drop_collection::MongoDropCollectionErrorNamed::MongoDB {
                    mongodb: e,
                    code_occurence: tufa_common::code_occurence!(),
                }
            )),
            Ok(client) => {
                let collection: mongodb::Collection<mongodb::bson::Document> =
                    client.database(db_name).collection(db_collection_name);
                if let Err(e) = collection.drop(None).await {
                    return Err(Box::new(
                        tufa_common::repositories_types::tufa_server::mongo_integration::mongo_drop_collection::MongoDropCollectionErrorNamed::MongoDB {
                            mongodb: e,
                            code_occurence: tufa_common::code_occurence!(),
                        }
                    ));
                }
                Ok(())
            }
        },
    }
}
