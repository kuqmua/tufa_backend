pub async fn mongo_drop_db<'a>(
    mongo_url: &'a str,
    db_name: &'a str,
) -> Result<(), Box<tufa_common::repositories_types::tufa_server::mongo_integration::mongo_drop_db::MongoDropDbErrorNamed<'a>>> {
    match  mongodb::options::ClientOptions::parse(mongo_url).await {
        Err(e) => Err(Box::new(
            tufa_common::repositories_types::tufa_server::mongo_integration::mongo_drop_db::MongoDropDbErrorNamed::MongoDB {
                mongodb: e,
                code_occurence: tufa_common::code_occurence!(),
            }
        )),
        Ok(client_options) => match mongodb::Client::with_options(client_options) {
            Err(e) => Err(Box::new(
                tufa_common::repositories_types::tufa_server::mongo_integration::mongo_drop_db::MongoDropDbErrorNamed::MongoDB {
                    mongodb: e,
                    code_occurence: tufa_common::code_occurence!(),
                }
            )),
            Ok(client) => {
                if let Err(e) = client.database(db_name).drop(None).await {
                    return Err(Box::new(
                        tufa_common::repositories_types::tufa_server::mongo_integration::mongo_drop_db::MongoDropDbErrorNamed::MongoDB {
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
