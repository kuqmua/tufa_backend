use mongodb::{bson::Document, options::ClientOptions, Client};

use crate::{
    config_mods::lazy_static_config::CONFIG,
    mongo_integration::mongo_get_documents_as_string_vector::MongoGetDocumentsAsStringVectorError,
    traits::provider_kind_trait::ProviderKindTrait,
};

use crate::mongo_integration::mongo_get_documents_as_string_vector::mongo_get_documents_as_string_vector;

use crate::{
    mongo_integration::mongo_get_db_url::mongo_get_db_url,
    providers::provider_kind_enum::ProviderKind,
};

#[derive(Debug)]
pub struct MongoGetProviderLinkPartsError {
    pub source: Box<MongoGetProviderLinkPartsErrorEnum>,
    line: String,
}

#[derive(Debug, ImplFromForUpperStruct)]
pub enum MongoGetProviderLinkPartsErrorEnum {
    ClientOptionsParse(ClientOptionsParseError),
    ClientWithOptions(ClientWithOptionsError),
    MongoGetDocumentsAsStringVector(MongoGetDocumentsAsStringVectorError),
}

#[derive(Debug)]
pub struct ClientOptionsParseError {
    pub source: mongodb::error::Error,
    line: String,
}

#[derive(Debug)]
pub struct ClientWithOptionsError {
    pub source: mongodb::error::Error,
    line: String,
}

impl ProviderKind {
    //rust does not support async traits yet (end of 2021). only with third party crate
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub async fn mongo_get_provider_link_parts(
        pk: ProviderKind,
    ) -> Result<Vec<String>, MongoGetProviderLinkPartsError> {
        match ClientOptions::parse(mongo_get_db_url()).await {
            Err(e) => {
                return Err(MongoGetProviderLinkPartsError {
                    source: Box::new(MongoGetProviderLinkPartsErrorEnum::ClientOptionsParse(
                        ClientOptionsParseError {
                            source: e,
                            line: format!("{} {}", line!().to_string(), file!().to_string()),
                        },
                    )),
                    line: format!("{} {}", line!().to_string(), file!().to_string()),
                })
            }
            Ok(client_options) => match Client::with_options(client_options) {
                Err(e) => {
                    return Err(MongoGetProviderLinkPartsError {
                        source: Box::new(MongoGetProviderLinkPartsErrorEnum::ClientWithOptions(
                            ClientWithOptionsError {
                                source: e,
                                line: format!("{} {}", line!().to_string(), file!().to_string()),
                            },
                        )),
                        line: format!("{} {}", line!().to_string(), file!().to_string()),
                    });
                }
                Ok(client) => {
                    match mongo_get_documents_as_string_vector(
                        client
                            .database(&CONFIG.mongo_providers_logs_db_name)
                            .collection::<Document>(&pk.get_mongo_log_collection_name()),
                        &CONFIG.mongo_providers_logs_db_collection_document_field_name_handle,
                        ProviderKind::get_mongo_provider_link_parts_aggregation(&pk),
                    )
                    .await
                    {
                        Err(e) => {
                            return Err(MongoGetProviderLinkPartsError {
                                    source: Box::new(
                                        MongoGetProviderLinkPartsErrorEnum::MongoGetDocumentsAsStringVector(e),
                                    ),
                                    line: format!("{} {}", line!().to_string(), file!().to_string())
                                });
                        }
                        Ok(vec_of_strings) => Ok(vec_of_strings),
                    }
                }
            },
        }
    }
}
