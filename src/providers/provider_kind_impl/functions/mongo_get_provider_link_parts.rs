use mongodb::{bson::Document, options::ClientOptions, Client};

use chrono::{DateTime, FixedOffset, Local, Utc};

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

use crate::helpers::where_was::WhereWas;

#[derive(Debug)]
pub struct MongoGetProviderLinkPartsError {
    pub source: Box<MongoGetProviderLinkPartsErrorEnum>,
}

#[derive(Debug)]
pub enum MongoGetProviderLinkPartsErrorEnum {
    ClientOptionsParse {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    ClientWithOptions {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    MongoGetDocumentsAsStringVector {
        source: MongoGetDocumentsAsStringVectorError,
        where_was: WhereWas,
    },
}

impl ProviderKind {
    //rust does not support async traits yet (end of 2021). only with third party crate
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub async fn mongo_get_provider_link_parts(
        pk: ProviderKind,
    ) -> Result<Vec<String>, MongoGetProviderLinkPartsError> {
        match ClientOptions::parse(mongo_get_db_url()).await {
            Err(e) => Err(MongoGetProviderLinkPartsError {
                source: Box::new(MongoGetProviderLinkPartsErrorEnum::ClientOptionsParse {
                    source: e,
                    where_was: WhereWas {
                        time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                            .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                }),
            }),
            Ok(client_options) => match Client::with_options(client_options) {
                Err(e) => Err(MongoGetProviderLinkPartsError {
                    source: Box::new(MongoGetProviderLinkPartsErrorEnum::ClientWithOptions {
                        source: e,
                        where_was: WhereWas {
                            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    }),
                }),
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
                        Err(e) => Err(MongoGetProviderLinkPartsError {
                            source: Box::new(
                                MongoGetProviderLinkPartsErrorEnum::MongoGetDocumentsAsStringVector {
                                    source: e,
                       where_was: WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
                        })}),
                        Ok(vec_of_strings) => Ok(vec_of_strings),
                    }
                }
            },
        }
    }
}
