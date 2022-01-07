use std::collections::HashMap;
use std::fmt;

use futures::future::join_all;

use mongodb::error::Error;

use crate::config_mods::lazy_static_config::CONFIG;

use crate::providers::provider_kind_enum::ProviderKind;

use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client,
};

use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;

#[derive(Debug, BoxErrFromErrDerive, ImplDisplayDerive)]
pub struct InitMongoError {
    /// check link status code error `{0}`
    pub source: Box<InitMongoErrorEnum>,
}

#[derive(Debug, ImplFromForUpperStruct)]
pub enum InitMongoErrorEnum {
    Client(Error),
    CollectionOperation((ProviderKind, Error)),
    CollectionIsNotEmpty((ProviderKind, u64)),
}

#[deny(clippy::indexing_slicing)]
pub async fn init_mongo(
    providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>,
) -> Result<(), InitMongoError> {
    let client_options = ClientOptions::parse(&mongo_get_db_url()).await?;
    match Client::with_options(client_options) {
        Err(e) => Err(InitMongoError {
            source: Box::new(InitMongoErrorEnum::Client(e)),
        }),
        Ok(client) => {
            let db = client.database(&CONFIG.mongo_providers_logs_db_name); //<- todo this is incorrect name
            let mut vec_of_futures = Vec::with_capacity(providers_json_local_data_hashmap.len());
            for pk in providers_json_local_data_hashmap.keys() {
                vec_of_futures.push(async {
                    (
                        *pk,
                        db.collection::<Document>(&format!("{}", *pk))
                            .count_documents(None, None)
                            .await,
                    )
                });
            }
            let result_vec = join_all(vec_of_futures).await;
            for (pk, result) in result_vec {
                match result {
                    //todo filter
                    Err(e) => {
                        return Err(InitMongoError {
                            source: Box::new(InitMongoErrorEnum::CollectionOperation((pk, e))),
                        })
                    }
                    Ok(documents_number) => {
                        if documents_number > 0 {
                            return Err(InitMongoError {
                                source: Box::new(InitMongoErrorEnum::CollectionIsNotEmpty((
                                    pk,
                                    documents_number,
                                ))),
                            });
                        }
                    }
                }
            }
            let mut vec_of_futures = Vec::with_capacity(providers_json_local_data_hashmap.len());
            for (pk, data_vec) in &providers_json_local_data_hashmap {
                vec_of_futures.push(async {
                            let pk_cloned = *pk;
                            let collection = db.collection(&format!("{}", pk_cloned));
                            let docs: Vec<Document> = data_vec.iter().map(|data| doc! { &CONFIG.mongo_providers_logs_db_collection_document_field_name_handle: data }).collect();
                            (pk_cloned, collection.insert_many(docs, None).await)
                        });
            }
            let result_vec = join_all(vec_of_futures).await;
            //todo: db partially initialized, print some warning
            for (pk, result) in result_vec {
                if let Err(e) = result {
                    return Err(InitMongoError {
                        source: Box::new(InitMongoErrorEnum::CollectionOperation((pk, e))),
                    });
                }
            }
            Ok(())
        }
    }
}
