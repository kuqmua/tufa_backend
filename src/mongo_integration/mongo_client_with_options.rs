pub fn mongo_client_with_options<'a>(client_options: mongodb::options::ClientOptions) -> Result<mongodb::Client, Box<tufa_common::repositories_types::tufa_server::mongo_integration::mongo_client_with_options::MongoClientWithOptionsOriginErrorNamed<'a>>>{
    match mongodb::Client::with_options(client_options) {
        Err(e) => Err(Box::new(
            tufa_common::repositories_types::tufa_server::mongo_integration::mongo_client_with_options::MongoClientWithOptionsOriginErrorNamed::Mongo { 
                error: e, 
                code_occurence: tufa_common::code_occurence!() 
            } 
        )),
        Ok(client) => Ok(client),
    }
}
