use std::fmt;

use mongodb::{
    bson::Document,
    options::ClientOptions,
    Client,
};

use crate::{
    config_mods::lazy_static_config::CONFIG, traits::provider_kind_trait::ProviderKindTrait, mongo_integration::mongo_get_documents_as_string_vector::MongoGetDocumentsAsStringVectorError,
};

use crate::mongo_integration::mongo_get_documents_as_string_vector::mongo_get_documents_as_string_vector;

use crate::{
    mongo_integration::mongo_get_db_url::mongo_get_db_url,
    providers::provider_kind_enum::ProviderKind,
};
use mongodb::Database;

#[derive(Debug, BoxErrFromErrDerive, ImplDisplayDerive)]
pub struct MongoGetProviderLinkPartsError {
    pub source: Box<MongoGetProviderLinkPartsErrorEnum>,
}

#[derive(Debug, ImplFromForUpperStruct)]
pub enum MongoGetProviderLinkPartsErrorEnum {
    ClientOptionsParse(ClientOptionsParseError),
    ClientWithOptions(ClientWithOptionsError),
    CountDocuments(CountDocumentsError),
    MongoGetDocumentsAsStringVector(MongoGetDocumentsAsStringVectorError),
}

#[derive(Debug)]
pub struct ClientOptionsParseError {
    pub source: Box<mongodb::error::Error>,
}

#[derive(Debug)]
pub struct ClientWithOptionsError {
    pub source: Box<mongodb::error::Error>,
}

#[derive(Debug)]
pub struct CountDocumentsError {
    pub source: Box<mongodb::error::Error>,
}

impl ProviderKind {
    //rust does not support async traits yet (end of 2021). only with  third party crate
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub async fn mongo_get_provider_link_parts(
        pk: ProviderKind,
    ) -> Result<Vec<String>, MongoGetProviderLinkPartsError> {
        //todo maybe option vec string
        //declare db name. there is no create db method in mongo
        let client_options: ClientOptions;
        match ClientOptions::parse(mongo_get_db_url()).await {
            Err(e) => {return Err(MongoGetProviderLinkPartsError {
                source: Box::new(MongoGetProviderLinkPartsErrorEnum::ClientOptionsParse(
                    ClientOptionsParseError{
                        source: Box::new(e)
                    }
                ))
            })},
            Ok(client_options_handle) => {client_options = client_options_handle;},
        }
        let db: Database;
        match Client::with_options(client_options) {
            Err(e) => {return Err(
                MongoGetProviderLinkPartsError {
                    source: Box::new(
                        MongoGetProviderLinkPartsErrorEnum::ClientWithOptions(
                            ClientWithOptionsError{
                                source: Box::new(e)
                            }
                        )
                    )
                }
            );},
            Ok(client) => {db = client.database(&CONFIG.mongo_providers_logs_db_name);},
        } 
        let collection = db.collection::<Document>(&pk.get_mongo_log_collection_name());
        let documents_number: u64;
        match collection.count_documents(None, None).await {
            Err(e) => {
                return Err(
                    MongoGetProviderLinkPartsError {
                        source: Box::new(
                            MongoGetProviderLinkPartsErrorEnum::CountDocuments(
                                CountDocumentsError{
                                    source: Box::new(e)
                                }
                            )
                        )
                    }
                );
            },
            Ok(number) => {documents_number = number;},
        }
        if documents_number > 0 {
            let vec_of_strings = mongo_get_documents_as_string_vector(
                collection,
                &CONFIG.mongo_providers_logs_db_collection_document_field_name_handle,
                ProviderKind::get_mongo_provider_link_parts_aggregation(&pk),
            )
            .await?;
            return Ok(vec_of_strings);
        }
        Ok(Vec::new())
    }
}
