use std::collections::HashMap;

use chrono::{DateTime, FixedOffset, Local, Utc};

use futures::future::join_all;

use crate::helpers::where_was::WhereWas;

use mongodb::{
    bson::{doc, Document},
    error::Error,
    options::ClientOptions,
    Client,
};

use crate::{
    config_mods::lazy_static_config::CONFIG, traits::provider_kind_trait::ProviderKindTrait,
};

use crate::providers::provider_kind_enum::ProviderKind;

use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;

#[derive(Debug)]
pub struct InitMongoError {
    pub source: Box<InitMongoErrorEnum>,
}

#[derive(Debug)]
pub enum InitMongoErrorEnum {
    ClientOptionsParse {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    ClientWithOptions {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    CollectionCountDocumentsOrIsNotEmpty {
        source: HashMap<ProviderKind, CollectionCountDocumentsOrIsNotEmpty>,
        where_was: WhereWas,
    },
    InsertManyError {
        source: HashMap<ProviderKind, Error>,
        where_was: WhereWas,
    },
}

#[derive(Debug)]
pub enum CollectionCountDocumentsOrIsNotEmpty {
    CountDocuments(Error),
    IsNotEmpty(u64),
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn init_mongo(
    providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>,
) -> Result<(), InitMongoError> {
    match ClientOptions::parse(&mongo_get_db_url()).await {
        Err(e) => Err(InitMongoError {
            source: Box::new(InitMongoErrorEnum::ClientOptionsParse {
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
            Err(e) => Err(InitMongoError {
                source: Box::new(InitMongoErrorEnum::ClientWithOptions {
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
                // let client_options = ClientOptions::parse(&mongo_get_db_url()).await?;
                // let client = Client::with_options(client_options)?;
                let db = client.database(&CONFIG.mongo_providers_link_parts_db_name);
                let error_vec_count_documents =
                    join_all(providers_json_local_data_hashmap.keys().map(|pk| async {
                        (
                            *pk,
                            db.collection::<Document>(&pk.get_db_tag())
                                .count_documents(None, None)
                                .await,
                        )
                    }))
                    .await
                    .into_iter()
                    .filter_map(|(pk, result)| match result {
                        Err(e) => {
                            Some((pk, CollectionCountDocumentsOrIsNotEmpty::CountDocuments(e)))
                        }
                        Ok(documents_number) => {
                            if documents_number > 0 {
                                return Some((
                                    pk,
                                    CollectionCountDocumentsOrIsNotEmpty::IsNotEmpty(
                                        documents_number,
                                    ),
                                ));
                            }
                            None
                        }
                    })
                    .collect::<HashMap<ProviderKind, CollectionCountDocumentsOrIsNotEmpty>>();
                if !error_vec_count_documents.is_empty() {
                    return Err(InitMongoError {
                        source: Box::new(
                            InitMongoErrorEnum::CollectionCountDocumentsOrIsNotEmpty {
                                source: error_vec_count_documents,
                                where_was: WhereWas {
                                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                    file: file!(),
                                    line: line!(),
                                    column: column!(),
                                },
                            },
                        ),
                    });
                }
                drop(error_vec_count_documents);
                let error_vec_insert_many = join_all(providers_json_local_data_hashmap.iter().map(|(pk, data_vec)| async {
                                        let docs: Vec<Document> = data_vec.iter().map(|data| doc! { &CONFIG.mongo_providers_logs_db_collection_document_field_name_handle: data }).collect();
                                        (*pk, db.collection(&pk.get_db_tag()).insert_many(docs, None).await)
                                    })).await
                    .into_iter()
                    .filter_map(|(pk, result)| {
                        if let Err(e) = result {
                            return Some((pk, e));
                        }
                        None
                    })
                    .collect::<HashMap<ProviderKind, Error>>();
                if !error_vec_insert_many.is_empty() {
                    return Err(InitMongoError {
                        source: Box::new(InitMongoErrorEnum::InsertManyError {
                            source: error_vec_insert_many,
                            where_was: WhereWas {
                                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            },
                        }),
                    });
                }
                Ok(())
            }
        },
    }
    //
}
