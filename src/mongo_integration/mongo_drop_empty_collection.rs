pub async fn mongo_drop_empty_collection<'a>(
    mongo_url: &'a str,
    db_name: &'a str,
    db_collection_name: &'a str,
) -> Result<(), Box<tufa_common::server::mongo::mongo_drop_empty_collection::MongoDropEmptyCollectionErrorNamed<'a>>> {
    match mongodb::options::ClientOptions::parse(mongo_url).await {
        Err(e) => Err(Box::new(
            tufa_common::server::mongo::mongo_drop_empty_collection::MongoDropEmptyCollectionErrorNamed::MongoDB {
                mongodb: e,
                code_occurence: tufa_common::code_occurence!(),
            }
        )),
        Ok(client_options) => {
            match mongodb::Client::with_options(client_options) {
                Err(e) => Err(Box::new(
                    tufa_common::server::mongo::mongo_drop_empty_collection::MongoDropEmptyCollectionErrorNamed::MongoDB {
                        mongodb: e,
                        code_occurence: tufa_common::code_occurence!(),
                    }
                )),
                Ok(client) => {
                    let collection: mongodb::Collection<mongodb::bson::Document> =
                        client.database(db_name).collection(db_collection_name);
                    match collection.count_documents(None, None).await {
                    Err(e) => Err(Box::new(
                        tufa_common::server::mongo::mongo_drop_empty_collection::MongoDropEmptyCollectionErrorNamed::MongoDB {
                            mongodb: e,
                            code_occurence: tufa_common::code_occurence!(),
                        }
                    )),
                    Ok(documents_number) => {
                        if documents_number > 0 {
                            Err(Box::new(
                                tufa_common::server::mongo::mongo_drop_empty_collection::MongoDropEmptyCollectionErrorNamed::CollectionIsNotEmpty {
                                    collection_name: db_collection_name,
                                    collection_len: documents_number,
                                    code_occurence: tufa_common::code_occurence!(),
                                }
                            ))
                        } else {
                            if let Err(e) = collection.drop(None).await {
                                return Err(Box::new(
                                    tufa_common::server::mongo::mongo_drop_empty_collection::MongoDropEmptyCollectionErrorNamed::MongoDB {
                                        mongodb: e,
                                        code_occurence: tufa_common::code_occurence!(),
                                    }
                                ));
                            }
                            Ok(())
                        }
                    }
                }
                }
            }
        }
    }
}
