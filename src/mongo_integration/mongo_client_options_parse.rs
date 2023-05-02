pub async fn mongo_client_options_parse<'a>() -> Result<mongodb::options::ClientOptions, Box<tufa_common::repositories_types::tufa_server::mongo_integration::mongo_client_options_parse::MongoClientOptionsParseOriginErrorNamed<'a>>>{
    match mongodb::options::ClientOptions::parse(&
        {
            use tufa_common::traits::get_mongo_url::GetMongoUrl;
            crate::global_variables::runtime::config::CONFIG.get_mongo_url()
        }
        ).await {
        Err(e) => Err(Box::new(
            tufa_common::repositories_types::tufa_server::mongo_integration::mongo_client_options_parse::MongoClientOptionsParseOriginErrorNamed::Mongo {
                error: e,
                code_occurence: tufa_common::code_occurence!(),
            },
        )),
        Ok(client_options) => Ok(client_options),
    }
}
